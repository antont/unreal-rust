#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCategoryFilter {
    pub category_name: FString,
    pub log_verbosity: i32,
    pub flags_20: u8,
}
#[repr(C, align(8))]
pub struct FVisualLoggerFiltersData {
    pub search_box_filter: FString,
    pub object_name_filter: FString,
    pub categories: TArray<FCategoryFilter>,
    pub selected_classes: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FVisualLoggerFilters {}
pub struct UVisualLoggerRenderingComponent {}
pub struct ULogVisualizerSessionSettings {
    pub b_enable_graphs_visualization: bool,
}
pub struct ULogVisualizerSettings {
    pub b_ignore_trivial_logs: bool,
    pub trivial_logs_threshold: i32,
    pub b_stick_to_recent_data: bool,
    pub b_reset_data_with_new_session: bool,
    pub b_show_histogram_labels_outside: bool,
    pub default_camera_distance: f32,
    pub b_search_inside_logs: bool,
    pub b_use_filter_volumes: bool,
    pub graphs_background_color: FColor,
    pub b_persistent_filters: bool,
    pub b_draw_extremes_on_graphs: bool,
    pub b_constrain_graph_to_local_min_max: bool,
    pub b_use_players_only_for_pause: bool,
    pub b_log_nav_octree_on_stop: bool,
    pub b_force_unique_log_names: bool,
    pub persistent_filters: FVisualLoggerFiltersData,
    pub debug_mesh_material_fake_light: UPtr<UMaterial>,
    pub debug_mesh_material_fake_light_name: FString,
}
pub struct AVisualLoggerCameraController {
    pub picked_actor: TWeakObjectPtr<AActor>,
}
pub struct AVisualLoggerHUD {}
pub struct AVisualLoggerRenderingActorBase {}
pub struct AVisualLoggerRenderingActor {}
