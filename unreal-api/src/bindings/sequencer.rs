#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FSequencerFilterSet {
    pub label: FString,
    pub enabled_states: TMap<FString, bool>,
    pub text_filter_string: FString,
}
#[repr(C, align(8))]
pub struct FSequencerFilterBarConfig {
    pub active_filters: FSequencerFilterSet,
    pub custom_text_filters: TArray<FCustomTextFilterData>,
    pub filter_bar_layout: EFilterBarLayout,
}
#[repr(C, align(8))]
pub struct FLevelEditorSequencerUpdateGizmoTickFunction {}
#[repr(C, align(1))]
pub struct FSequencerThumbnailCaptureSettings {
    pub capture_frame_location_rule: ESequencerThumbnailCaptureTimeLocation,
}
#[repr(C, align(4))]
pub struct FColumnVisibilitySetting {
    pub column_name: FName,
    pub b_is_visible: bool,
}
#[repr(C, align(8))]
pub struct FMovieScenePasteFoldersParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub parent_folder: UPtr<UMovieSceneFolder>,
}
#[repr(C, align(8))]
pub struct FMovieScenePasteSectionsParams {
    pub tracks: TArray<UPtr<UMovieSceneTrack>>,
    pub track_row_indices: TArray<i32>,
    pub time: FFrameTime,
}
#[repr(C, align(8))]
pub struct FMovieScenePasteTracksParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub bindings: TArray<FMovieSceneBindingProxy>,
    pub parent_folder: UPtr<UMovieSceneFolder>,
    pub folders: TArray<UPtr<UMovieSceneFolder>>,
}
#[repr(C, align(8))]
pub struct FMovieScenePasteBindingsParams {
    pub bindings: TArray<FMovieSceneBindingProxy>,
    pub parent_folder: UPtr<UMovieSceneFolder>,
    pub folders: TArray<UPtr<UMovieSceneFolder>>,
    pub b_duplicate_existing_actors: bool,
    pub pasted_actors: TMap<FName, UPtr<AActor>>,
}
#[repr(C, align(4))]
pub struct FSequencerChangeBindingInfo {
    pub binding_id: FGuid,
    pub binding_index: i32,
}
pub struct UMotionTrailToolOptions {
    pub b_show_trails: bool,
    pub b_show_selected_trails: bool,
    pub trail_style: EMotionTrailTrailStyle,
    pub default_color: FLinearColor,
    pub time_pre_color: FLinearColor,
    pub time_post_color: FLinearColor,
    pub dash_pre_color: FLinearColor,
    pub dash_post_color: FLinearColor,
    pub b_show_full_trail: bool,
    pub trail_thickness: f64,
    pub frames_before: i32,
    pub frames_after: i32,
    pub b_show_keys: bool,
    pub b_show_frame_number_deprecated: bool,
    pub key_color: FLinearColor,
    pub selected_key_color: FLinearColor,
    pub key_size: f64,
    pub b_show_marks: bool,
    pub mark_color: FLinearColor,
    pub mark_size: f64,
    pub max_number_pinned: i32,
    pub b_lock_marks_to_frames_deprecated: bool,
    pub seconds_per_mark_deprecated: f64,
}
pub struct UMovieSceneCopyableBinding {
    pub spawnable_object_templates: TArray<UPtr<UObject>>,
    pub num_spawnable_object_templates: i32,
    pub tracks: TArray<UPtr<UMovieSceneTrack>>,
    pub binding: FMovieSceneBinding,
    pub spawnable: FMovieSceneSpawnable,
    pub possessable: FMovieScenePossessable,
    pub bound_object_names: TArray<FString>,
    pub folder_path: TArray<FName>,
    pub tags: TArray<FName>,
    pub custom_bindings: TArray<UPtr<UMovieSceneCustomBinding>>,
    pub num_custom_bindings: i32,
    pub preview_spawnable_bindings: TArray<i32>,
}
pub struct UMovieSceneCopyableTrack {
    pub track: UPtr<UMovieSceneTrack>,
    pub b_is_root_track: bool,
    pub b_is_camera_cut_track: bool,
    pub folder_path: TArray<FName>,
}
pub struct USequencerFilterBarContext {}
pub struct USequencerFilterMenuContext {}
pub struct USequencerMenuContext {}
pub struct USequencerToolMenuContext {}
pub struct USequencerClockSourceMenuContext {}
pub struct USequencerTimeSliderControllerMenuContext {}
pub struct USequencerTrackFilterExtension {}
pub struct USequencerTrackFilterTextExpressionExtension {}
pub struct USequencerModuleOutlinerScriptingObject {}
pub struct USequencerModuleScriptingLayer {}
pub struct UMovieSceneKeyStructType {
    pub source_times_property: TFieldPath<FArrayProperty>,
    pub source_values_property: TFieldPath<FArrayProperty>,
    pub dest_time_property: TFieldPath<FStructProperty>,
    pub dest_value_property: TFieldPath<FProperty>,
}
pub struct USequencerSettingsContainer {}
pub struct USequencerSettings {
    pub auto_change_mode: EAutoChangeMode,
    pub allow_edits_mode: EAllowEditsMode,
    pub key_group_mode: EKeyGroupMode,
    pub key_interpolation: EMovieSceneKeyInterpolation,
    pub b_auto_set_track_defaults: bool,
    pub spawn_position: ESequencerSpawnPosition,
    pub b_create_spawnable_cameras: bool,
    pub b_show_range_slider: bool,
    pub b_is_snap_enabled: bool,
    pub b_snap_key_times_to_elements: bool,
    pub b_snap_section_times_to_elements: bool,
    pub b_snap_play_time_to_keys: bool,
    pub b_snap_play_time_to_sections: bool,
    pub b_snap_play_time_to_markers: bool,
    pub b_snap_play_time_to_pressed_key: bool,
    pub b_snap_play_time_to_dragged_key: bool,
    pub b_snap_curve_value_to_interval: bool,
    pub b_force_whole_frames: bool,
    pub b_show_selected_nodes_only: bool,
    pub b_rewind_on_record: bool,
    pub b_left_mouse_drag_does_marquee: bool,
    pub zoom_position: ESequencerZoomPosition,
    pub b_auto_scroll_enabled: bool,
    pub b_stop_playing_when_jumping_to_start_or_end: bool,
    pub b_scrub_time_start_from_cursor: bool,
    pub b_link_curve_editor_time_range: bool,
    pub b_link_filters_with_curve_editor: bool,
    pub b_synchronize_curve_editor_selection: bool,
    pub b_isolate_curve_editor_to_selection: bool,
    pub b_curve_editor_visible: bool,
    pub curve_editor_zoom_scaling: FCurveEditorZoomScaleConfig,
    pub loop_mode: ESequencerLoopMode,
    pub b_reset_playhead_when_navigating: bool,
    pub b_keep_cursor_in_play_range_while_scrubbing: bool,
    pub b_keep_play_range_in_section_bounds: bool,
    pub zero_pad_frames: u8,
    pub jump_frame_increment: FFrameNumber,
    pub time_warp_display: ESequencerTimeWarpDisplay,
    pub b_show_layer_bars: bool,
    pub b_show_key_bars: bool,
    pub b_infinite_key_areas: bool,
    pub b_show_channel_colors: bool,
    pub b_show_info_button: bool,
    pub b_show_tick_lines: bool,
    pub b_show_sequencer_toolbar: bool,
    pub b_show_marked_frames: bool,
    pub b_show_scaling_anchors: bool,
    pub key_area_curve_extents: FString,
    pub key_area_height_with_curves: f32,
    pub reduce_keys_tolerance: f32,
    pub b_delete_keys_when_trimming: bool,
    pub b_disable_sections_after_baking: bool,
    pub playback_range_start_color: FLinearColor,
    pub playback_range_end_color: FLinearColor,
    pub marked_frame_color: FLinearColor,
    pub section_color_tints: TArray<FColor>,
    pub b_clean_playback_mode: bool,
    pub b_activate_realtime_viewports: bool,
    pub b_evaluate_sub_sequences_in_isolation: bool,
    pub b_rerun_construction_scripts: bool,
    pub b_show_debug_visualization: bool,
    pub b_visualize_pre_and_post_roll: bool,
    pub b_compile_director_on_evaluate: bool,
    pub trajectory_path_cap: u32,
    pub frame_number_display_format: EFrameNumberDisplayFormats,
    pub movie_renderer_name: FString,
    pub b_auto_expand_nodes_on_selection: bool,
    pub b_restore_original_viewport_on_camera_cut_unlock: bool,
    pub tree_view_width: f32,
    pub view_density: FName,
    pub asset_browser_width: f32,
    pub asset_browser_height: f32,
    pub track_filters: TArray<FString>,
    pub column_visibility_settings: TArray<FColumnVisibilitySetting>,
    pub sidebar_state: TMap<FName, FSidebarState>,
    pub track_filter_bars: TMap<FName, FSequencerFilterBarConfig>,
    pub b_include_pinned_in_filter: bool,
    pub b_auto_expand_nodes_on_filter_pass: bool,
    pub b_use_filter_submenus_for_categories: bool,
    pub b_filter_bar_visible: bool,
    pub last_filter_bar_layout: EFilterBarLayout,
    pub last_filter_bar_size_coefficient: f32,
    pub thumbnail_capture_settings: FSequencerThumbnailCaptureSettings,
    pub b_navigation_tool_visible: bool,
}
pub struct USequencerTimeChangeUndoRedoProxy {
    pub time: FQualifiedFrameTime,
    pub b_time_was_set: bool,
}
