#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_level_sequence_editor_blueprint_library_set_track_filter_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_track_filter_active: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_selection_range_start: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_selection_range_end: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_random_color_for_channels: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_playback_speed: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_loop_mode: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_lock_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_lock_camera_cut_to_viewport: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_local_position: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_global_position: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_custom_color_for_channels: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_custom_color_for_channel: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_current_time: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_set_current_local_time: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_select_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_select_sections: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_select_keys: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_select_folders: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_select_channels: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_select_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_refresh_current_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_play_to: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_play: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_pause: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_open_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_is_track_filter_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_is_track_filter_active: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_is_playing: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_is_level_sequence_locked: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_is_camera_cut_locked_to_viewport: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_has_custom_color_for_channel: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_track_filter_names: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_sub_sequence_hierarchy: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selection_range_start: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selection_range_end: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selected_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selected_sections: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selected_keys: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selected_folders: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selected_channels: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_selected_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_playback_start_position: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_playback_speed: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_playback_end_position: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_loop_mode: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_local_position: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_global_position: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_focused_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_custom_color_for_channel: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_current_time: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_current_local_time: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_current_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_channels_with_selected_keys: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_get_bound_objects: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_force_update: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_focus_parent_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_focus_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_empty_selection: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_deselect_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_delete_color_for_channels: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_blueprint_library_close_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_sync_sections_using_source_timecode: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_snap_sections_to_timeline_using_source_timecode: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_save_default_spawnable_state: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_replace_binding_with_actors: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_remove_invalid_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_remove_all_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_remove_actors_from_binding: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_rebind_component: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_paste_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_paste_sections: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_paste_folders: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_paste_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_get_scripting_layer: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_get_custom_binding_type: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_get_custom_bindings_of_type: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_get_custom_binding_objects: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_get_curve_editor: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_fix_actor_references: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_create_camera: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_copy_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_copy_sections: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_copy_folders: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_copy_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_convert_to_spawnable: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_convert_to_possessable: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_convert_to_custom_binding: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_change_actor_template_class: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_bake_transform_with_settings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_add_spawnable_from_instance: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_add_spawnable_from_class: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_add_actors_to_binding: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_editor_subsystem_add_actors: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_level_sequence_editor_blueprint_library_set_track_filter_enabled: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_track_filter_active: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_selection_range_start: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_selection_range_end: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_random_color_for_channels: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_playback_speed: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_loop_mode: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_lock_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_lock_camera_cut_to_viewport: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_local_position: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_global_position: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_custom_color_for_channels: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_custom_color_for_channel: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_current_time: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_set_current_local_time: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_select_tracks: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_select_sections: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_select_keys: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_select_folders: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_select_channels: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_select_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_refresh_current_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_play_to: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_play: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_pause: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_open_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_is_track_filter_enabled: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_is_track_filter_active: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_is_playing: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_is_level_sequence_locked: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_is_camera_cut_locked_to_viewport: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_has_custom_color_for_channel: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_track_filter_names: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_sub_sequence_hierarchy: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selection_range_start: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selection_range_end: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selected_tracks: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selected_sections: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selected_keys: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selected_folders: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selected_channels: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_selected_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_playback_start_position: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_playback_speed: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_playback_end_position: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_loop_mode: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_local_position: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_global_position: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_focused_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_custom_color_for_channel: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_current_time: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_current_local_time: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_current_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_channels_with_selected_keys: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_get_bound_objects: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_force_update: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_focus_parent_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_focus_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_empty_selection: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_deselect_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_delete_color_for_channels: std::ptr::null_mut(),
            u_level_sequence_editor_blueprint_library_close_level_sequence: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_sync_sections_using_source_timecode: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_snap_sections_to_timeline_using_source_timecode: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_save_default_spawnable_state: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_replace_binding_with_actors: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_remove_invalid_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_remove_all_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_remove_actors_from_binding: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_rebind_component: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_paste_tracks: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_paste_sections: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_paste_folders: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_paste_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_get_scripting_layer: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_get_custom_binding_type: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_get_custom_bindings_of_type: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_get_custom_binding_objects: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_get_curve_editor: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_fix_actor_references: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_create_camera: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_copy_tracks: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_copy_sections: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_copy_folders: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_copy_bindings: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_convert_to_spawnable: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_convert_to_possessable: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_convert_to_custom_binding: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_change_actor_template_class: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_bake_transform_with_settings: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_add_spawnable_from_instance: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_add_spawnable_from_class: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_add_actors_to_binding: std::ptr::null_mut(),
            u_level_sequence_editor_subsystem_add_actors: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceEditorBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrackFilterEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_track_filter_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrackFilterActive"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_track_filter_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectionRangeStart"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_selection_range_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectionRangeEnd"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_selection_range_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRandomColorForChannels"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_random_color_for_channels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackSpeed"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_playback_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLoopMode"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_loop_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLockLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_lock_level_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLockCameraCutToViewport"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_lock_camera_cut_to_viewport,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalPosition"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_local_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGlobalPosition"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_global_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomColorForChannels"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_custom_color_for_channels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomColorForChannel"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_custom_color_for_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentTime"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_current_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentLocalTime"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_set_current_local_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectTracks"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_select_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectSections"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_select_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectKeys"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_select_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectFolders"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_select_folders,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectChannels"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_select_channels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectBindings"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_select_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshCurrentLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_refresh_current_level_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayTo"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_blueprint_library_play_to,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_blueprint_library_play,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pause"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_blueprint_library_pause,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_open_level_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTrackFilterEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_is_track_filter_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTrackFilterActive"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_is_track_filter_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaying"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_blueprint_library_is_playing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLevelSequenceLocked"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_is_level_sequence_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraCutLockedToViewport"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_is_camera_cut_locked_to_viewport,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasCustomColorForChannel"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_has_custom_color_for_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackFilterNames"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_track_filter_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSubSequenceHierarchy"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_sub_sequence_hierarchy,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionRangeStart"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selection_range_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionRangeEnd"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selection_range_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedTracks"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selected_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedSections"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selected_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedKeys"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selected_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedFolders"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selected_folders,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedChannels"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selected_channels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedBindings"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_selected_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackStartPosition"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_playback_start_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackSpeed"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_playback_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackEndPosition"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_playback_end_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoopMode"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_loop_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalPosition"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_local_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalPosition"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_global_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFocusedLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_focused_level_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomColorForChannel"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_custom_color_for_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentTime"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_current_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentLocalTime"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_current_local_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_current_level_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelsWithSelectedKeys"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_channels_with_selected_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObjects"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_get_bound_objects,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceUpdate"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_force_update,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FocusParentSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_focus_parent_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FocusLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_focus_level_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmptySelection"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_empty_selection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeselectBindings"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_deselect_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteColorForChannels"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_delete_color_for_channels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CloseLevelSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_blueprint_library_close_level_sequence,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SyncSectionsUsingSourceTimecode"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_sync_sections_using_source_timecode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SnapSectionsToTimelineUsingSourceTimecode"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_snap_sections_to_timeline_using_source_timecode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveDefaultSpawnableState"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_save_default_spawnable_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceBindingWithActors"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_replace_binding_with_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveInvalidBindings"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_remove_invalid_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllBindings"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_remove_all_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorsFromBinding"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_remove_actors_from_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RebindComponent"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_rebind_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PasteTracks"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_paste_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PasteSections"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_paste_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PasteFolders"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_paste_folders,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PasteBindings"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_paste_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScriptingLayer"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_get_scripting_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBindingType"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_get_custom_binding_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBindingsOfType"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_get_custom_bindings_of_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBindingObjects"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_get_custom_binding_objects,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveEditor"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_get_curve_editor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FixActorReferences"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_fix_actor_references,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateCamera"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_create_camera,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyTracks"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_copy_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopySections"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_copy_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyFolders"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_copy_folders,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyBindings"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_copy_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSpawnable"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_convert_to_spawnable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToPossessable"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_convert_to_possessable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToCustomBinding"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_convert_to_custom_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ChangeActorTemplateClass"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_change_actor_template_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BakeTransformWithSettings"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_bake_transform_with_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSpawnableFromInstance"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_add_spawnable_from_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSpawnableFromClass"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_add_spawnable_from_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorsToBinding"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_editor_subsystem_add_actors_to_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActors"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_editor_subsystem_add_actors,
        );
    }
}
#[repr(C, align(1))]
pub struct FMovieSceneScriptingParams {
    pub time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
}
impl FMovieSceneScriptingParams {}
#[repr(C, align(8))]
pub struct UCinematicLevelViewportToolbarContext {
    __padding_end: [u8; 64],
}
impl UCinematicLevelViewportToolbarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCinematicLevelViewportToolbarContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorMenuContext {
    __padding_end: [u8; 64],
}
impl ULevelSequenceEditorMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceEditorMenuContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_LevelSequence {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_LevelSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_LevelSequence")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceFactoryNew {
    __padding_end: [u8; 136],
}
impl ULevelSequenceFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceFactoryNew")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFilmOverlayToolkit {
    __padding_end: [u8; 48],
}
impl UFilmOverlayToolkit {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFilmOverlayToolkit")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULevelSequenceEditorBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceEditorBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_track_filter_enabled(track_filter_name: &FText, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_track_filter_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                track_filter_name,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_track_filter_enabled,
                __buffer,
            )
        };
    }
    pub fn set_track_filter_active(track_filter_name: &FText, b_active: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_track_filter_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                track_filter_name,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_active, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_track_filter_active,
                __buffer,
            )
        };
    }
    pub fn set_selection_range_start(new_frame: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_selection_range_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_frame, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_selection_range_start,
                __buffer,
            )
        };
    }
    pub fn set_selection_range_end(new_frame: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_selection_range_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_frame, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_selection_range_end,
                __buffer,
            )
        };
    }
    pub fn set_random_color_for_channels(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifiers: &TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_random_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                identifiers,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_random_color_for_channels,
                __buffer,
            )
        };
    }
    pub fn set_playback_speed(new_playback_speed: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_playback_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_playback_speed,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_playback_speed,
                __buffer,
            )
        };
    }
    pub fn set_loop_mode(new_loop_mode: crate::bindings::sequencer::ESequencerLoopMode) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_loop_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_loop_mode,
                __buffer.add(0).cast::<crate::bindings::sequencer::ESequencerLoopMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_loop_mode,
                __buffer,
            )
        };
    }
    pub fn set_lock_level_sequence(b_lock: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_lock_level_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_lock, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_lock_level_sequence,
                __buffer,
            )
        };
    }
    pub fn set_lock_camera_cut_to_viewport(b_lock: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_lock_camera_cut_to_viewport,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_lock, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_lock_camera_cut_to_viewport,
                __buffer,
            )
        };
    }
    pub fn set_local_position(
        playback_params: crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_local_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &playback_params,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_local_position,
                __buffer,
            )
        };
    }
    pub fn set_global_position(
        playback_params: crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_global_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &playback_params,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_global_position,
                __buffer,
            )
        };
    }
    pub fn set_custom_color_for_channels(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifiers: &TArray<FString>,
        new_colors: &TArray<crate::bindings::core_u_object::FLinearColor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_custom_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                identifiers,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_colors,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_custom_color_for_channels,
                __buffer,
            )
        };
    }
    pub fn set_custom_color_for_channel(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: FString,
        new_color: &crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_color,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_custom_color_for_channel,
                __buffer,
            )
        };
    }
    pub fn set_current_time(new_frame: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_current_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_frame, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_current_time,
                __buffer,
            )
        };
    }
    pub fn set_current_local_time(new_frame: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_current_local_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_frame, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_set_current_local_time,
                __buffer,
            )
        };
    }
    pub fn select_tracks(
        tracks: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tracks,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_tracks,
                __buffer,
            )
        };
    }
    pub fn select_sections(
        sections: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sections,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_sections,
                __buffer,
            )
        };
    }
    pub fn select_keys(
        channel: &crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
        indices: &TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                indices,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_keys,
                __buffer,
            )
        };
    }
    pub fn select_folders(
        folders: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_folders,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                folders,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_folders,
                __buffer,
            )
        };
    }
    pub fn select_channels(
        channels: &TArray<
            crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channels,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_channels,
                __buffer,
            )
        };
    }
    pub fn select_bindings(
        object_bindings: &TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_bindings,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_select_bindings,
                __buffer,
            )
        };
    }
    pub fn refresh_current_level_sequence() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_refresh_current_level_sequence,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_refresh_current_level_sequence,
                __buffer,
            )
        };
    }
    pub fn play_to(
        playback_params: crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_play_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &playback_params,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_play_to,
                __buffer,
            )
        };
    }
    pub fn play() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_play,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_play,
                __buffer,
            )
        };
    }
    pub fn pause() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_pause,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_pause,
                __buffer,
            )
        };
    }
    pub fn open_level_sequence(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_open_level_sequence,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_open_level_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_track_filter_enabled(track_filter_name: &FText) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_track_filter_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                track_filter_name,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_track_filter_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_track_filter_active(track_filter_name: &FText) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_track_filter_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                track_filter_name,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_track_filter_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_playing() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_playing,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_level_sequence_locked() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_level_sequence_locked,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_level_sequence_locked,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_camera_cut_locked_to_viewport() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_camera_cut_locked_to_viewport,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_is_camera_cut_locked_to_viewport,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_custom_color_for_channel(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_has_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_has_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_track_filter_names() -> TArray<FText> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_track_filter_names,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_track_filter_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FText>>().read() }
    }
    pub fn get_sub_sequence_hierarchy() -> TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_sub_sequence_hierarchy,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_sub_sequence_hierarchy,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>>,
                >()
                .read()
        }
    }
    pub fn get_selection_range_start() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selection_range_start,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selection_range_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_selection_range_end() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selection_range_end,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selection_range_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_selected_tracks() -> TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_tracks,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_tracks,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn get_selected_sections() -> TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_sections,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_sections,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>>()
                .read()
        }
    }
    pub fn get_selected_keys(
        channel_proxy: &crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel_proxy,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<i32>>().read() }
    }
    pub fn get_selected_folders() -> TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_folders,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_folders,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>>()
                .read()
        }
    }
    pub fn get_selected_channels() -> TArray<
        crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_channels,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_channels,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<
                        crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
                    >,
                >()
                .read()
        }
    }
    pub fn get_selected_bindings() -> TArray<
        crate::bindings::movie_scene::FMovieSceneBindingProxy,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_bindings,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_selected_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
    pub fn get_playback_start_position(
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_playback_start_position,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_playback_start_position,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
                >()
                .read()
        }
    }
    pub fn get_playback_speed() -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_playback_speed,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_playback_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_playback_end_position(
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_playback_end_position,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_playback_end_position,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
                >()
                .read()
        }
    }
    pub fn get_loop_mode() -> crate::bindings::sequencer::ESequencerLoopMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_loop_mode,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_loop_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::sequencer::ESequencerLoopMode>()
                .read()
        }
    }
    pub fn get_local_position(
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_local_position,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_local_position,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
                >()
                .read()
        }
    }
    pub fn get_global_position(
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_global_position,
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
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_global_position,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    crate::bindings::movie_scene::FMovieSceneSequencePlaybackParams,
                >()
                .read()
        }
    }
    pub fn get_focused_level_sequence() -> UPtr<
        crate::bindings::level_sequence::ULevelSequence,
    > {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_focused_level_sequence,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_focused_level_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>()
                .read()
        }
    }
    pub fn get_custom_color_for_channel(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: FString,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_custom_color_for_channel,
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
    pub fn get_current_time() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_current_time,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_current_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_current_local_time() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_current_local_time,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_current_local_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_current_level_sequence() -> UPtr<
        crate::bindings::level_sequence::ULevelSequence,
    > {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_current_level_sequence,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_current_level_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>()
                .read()
        }
    }
    pub fn get_channels_with_selected_keys() -> TArray<
        crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_channels_with_selected_keys,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_channels_with_selected_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<
                        crate::bindings::sequencer_scripting_editor::FSequencerChannelProxy,
                    >,
                >()
                .read()
        }
    }
    pub fn get_bound_objects(
        object_binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_bound_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_get_bound_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn force_update() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_force_update,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_force_update,
                __buffer,
            )
        };
    }
    pub fn focus_parent_sequence() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_focus_parent_sequence,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_focus_parent_sequence,
                __buffer,
            )
        };
    }
    pub fn focus_level_sequence(
        sub_section: UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_focus_level_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sub_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_focus_level_sequence,
                __buffer,
            )
        };
    }
    pub fn empty_selection() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_empty_selection,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_empty_selection,
                __buffer,
            )
        };
    }
    pub fn deselect_bindings(
        object_bindings: &TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_deselect_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_bindings,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_deselect_bindings,
                __buffer,
            )
        };
    }
    pub fn delete_color_for_channels(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_delete_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_delete_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(identifier);
        }
    }
    pub fn close_level_sequence() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_close_level_sequence,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence_editor::ULevelSequenceEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_blueprint_library_close_level_sequence,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBindingPropertyInfoList {
    __padding_end: [u8; 64],
}
impl UMovieSceneBindingPropertyInfoList {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingPropertyInfoList")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorSubsystem {
    __padding_end: [u8; 328],
}
impl ULevelSequenceEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceEditorSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn sync_sections_using_source_timecode(
        &mut self,
        sections: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_sync_sections_using_source_timecode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sections,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_sync_sections_using_source_timecode,
                __buffer,
            )
        };
    }
    pub fn snap_sections_to_timeline_using_source_timecode(
        &mut self,
        sections: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_snap_sections_to_timeline_using_source_timecode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sections,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_snap_sections_to_timeline_using_source_timecode,
                __buffer,
            )
        };
    }
    pub fn save_default_spawnable_state(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_save_default_spawnable_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_save_default_spawnable_state,
                __buffer,
            )
        };
    }
    pub fn replace_binding_with_actors(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_replace_binding_with_actors,
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
                object_binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_replace_binding_with_actors,
                __buffer,
            )
        };
    }
    pub fn remove_invalid_bindings(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_remove_invalid_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_remove_invalid_bindings,
                __buffer,
            )
        };
    }
    pub fn remove_all_bindings(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_remove_all_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_remove_all_bindings,
                __buffer,
            )
        };
    }
    pub fn remove_actors_from_binding(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_remove_actors_from_binding,
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
                object_binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_remove_actors_from_binding,
                __buffer,
            )
        };
    }
    pub fn rebind_component(
        &mut self,
        component_bindings: &TArray<
            crate::bindings::movie_scene::FMovieSceneBindingProxy,
        >,
        component_name: &FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_rebind_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                component_bindings,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                component_name,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_rebind_component,
                __buffer,
            )
        };
    }
    pub fn paste_tracks(
        &mut self,
        text_to_import: FString,
        paste_tracks_params: crate::bindings::sequencer::FMovieScenePasteTracksParams,
        out_tracks: &mut TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &text_to_import,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &paste_tracks_params,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::sequencer::FMovieScenePasteTracksParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_tracks,
                __buffer
                    .add(64)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_tracks,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .swap(out_tracks);
        }
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn paste_sections(
        &mut self,
        text_to_import: FString,
        paste_sections_params: crate::bindings::sequencer::FMovieScenePasteSectionsParams,
        out_sections: &mut TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &text_to_import,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &paste_sections_params,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::sequencer::FMovieScenePasteSectionsParams,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_sections,
                __buffer
                    .add(56)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_sections,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>>()
                .swap(out_sections);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn paste_folders(
        &mut self,
        text_to_import: FString,
        paste_folders_params: crate::bindings::sequencer::FMovieScenePasteFoldersParams,
        out_folders: &mut TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_folders,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &text_to_import,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &paste_folders_params,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::sequencer::FMovieScenePasteFoldersParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_folders,
                __buffer
                    .add(32)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_folders,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>>()
                .swap(out_folders);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn paste_bindings(
        &mut self,
        text_to_import: FString,
        paste_bindings_params: crate::bindings::sequencer::FMovieScenePasteBindingsParams,
        out_object_bindings: &mut TArray<
            crate::bindings::movie_scene::FMovieSceneBindingProxy,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<161>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &text_to_import,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &paste_bindings_params,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::sequencer::FMovieScenePasteBindingsParams,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_object_bindings,
                __buffer
                    .add(144)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_paste_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(144)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .swap(out_object_bindings);
        }
        unsafe { __buffer.add(160).cast::<bool>().read() }
    }
    pub fn get_scripting_layer(
        &mut self,
    ) -> UPtr<crate::bindings::sequencer::USequencerModuleScriptingLayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_scripting_layer,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_scripting_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<crate::bindings::sequencer::USequencerModuleScriptingLayer>,
                >()
                .read()
        }
    }
    pub fn get_custom_binding_type(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TSubclassOf<crate::bindings::movie_scene::UMovieSceneCustomBinding> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_custom_binding_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_custom_binding_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TSubclassOf<crate::bindings::movie_scene::UMovieSceneCustomBinding>,
                >()
                .read()
        }
    }
    pub fn get_custom_bindings_of_type(
        &mut self,
        custom_binding_type: TSubclassOf<
            crate::bindings::movie_scene::UMovieSceneCustomBinding,
        >,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_custom_bindings_of_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_binding_type,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<
                            crate::bindings::movie_scene::UMovieSceneCustomBinding,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_custom_bindings_of_type,
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
    pub fn get_custom_binding_objects(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneCustomBinding>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_custom_binding_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_custom_binding_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TArray<UPtr<crate::bindings::movie_scene::UMovieSceneCustomBinding>>,
                >()
                .read()
        }
    }
    pub fn get_curve_editor(
        &mut self,
    ) -> UPtr<crate::bindings::sequencer_scripting_editor::USequencerCurveEditorObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_curve_editor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_get_curve_editor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<
                        crate::bindings::sequencer_scripting_editor::USequencerCurveEditorObject,
                    >,
                >()
                .read()
        }
    }
    pub fn fix_actor_references(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_fix_actor_references,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_fix_actor_references,
                __buffer,
            )
        };
    }
    pub fn create_camera(
        &mut self,
        b_spawnable: bool,
        out_actor: &mut UPtr<crate::bindings::cinematic_camera::ACineCameraActor>,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_create_camera,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_spawnable,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actor,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::cinematic_camera::ACineCameraActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_create_camera,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::cinematic_camera::ACineCameraActor>>()
                .swap(out_actor);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn copy_tracks(
        &mut self,
        tracks: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
        exported_text: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tracks,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                exported_text,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_tracks,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(exported_text);
        }
    }
    pub fn copy_sections(
        &mut self,
        sections: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
        exported_text: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sections,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                exported_text,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_sections,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(exported_text);
        }
    }
    pub fn copy_folders(
        &mut self,
        folders: &TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
        folders_exported_text: &mut FString,
        objects_exported_text: &mut FString,
        tracks_exported_text: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_folders,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                folders,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                folders_exported_text,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                objects_exported_text,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tracks_exported_text,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_folders,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(folders_exported_text);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(objects_exported_text);
        }
        unsafe {
            __buffer.add(48).cast::<FString>().swap(tracks_exported_text);
        }
    }
    pub fn copy_bindings(
        &mut self,
        bindings: &TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
        exported_text: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                bindings,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                exported_text,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_copy_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(exported_text);
        }
    }
    pub fn convert_to_spawnable(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_convert_to_spawnable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_convert_to_spawnable,
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
    pub fn convert_to_possessable(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_convert_to_possessable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_convert_to_possessable,
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
    pub fn convert_to_custom_binding(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        binding_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneCustomBinding>,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_convert_to_custom_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding_type,
                __buffer
                    .add(24)
                    .cast::<
                        TSubclassOf<
                            crate::bindings::movie_scene::UMovieSceneCustomBinding,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_convert_to_custom_binding,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn change_actor_template_class(
        &mut self,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_change_actor_template_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(24).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_change_actor_template_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn bake_transform_with_settings(
        &mut self,
        object_bindings: &TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
        in_settings: &crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
        params: &FMovieSceneScriptingParams,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<46>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_bake_transform_with_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_bindings,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(44).cast::<FMovieSceneScriptingParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_bake_transform_with_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(45).cast::<bool>().read() }
    }
    pub fn add_spawnable_from_instance(
        &mut self,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_spawnable_from_instance,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_spawnable_from_instance,
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
        &mut self,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_spawnable_from_class,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_spawnable_from_class,
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
    pub fn add_actors_to_binding(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        object_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_actors_to_binding,
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
                object_binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_actors_to_binding,
                __buffer,
            )
        };
    }
    pub fn add_actors(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_actors,
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
                crate::bindings::level_sequence_editor::__FUNCTION_PTRS
                    .u_level_sequence_editor_subsystem_add_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorSettings {
    __padding_end: [u8; 72],
}
impl ULevelSequenceEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceEditorSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceWithShotsSettings {
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 128],
    pub sub_sequence_names: TArray<FName>,
    __padding_end: [u8; 8],
}
impl ULevelSequenceWithShotsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceWithShotsSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
