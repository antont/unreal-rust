#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FVisibleColumnsSettings {}
#[repr(C, align(8))]
pub struct FAudioEventLogCustomEvents {
    pub event_names: TSet<FString>,
}
#[repr(C, align(8))]
pub struct FAudioEventLogVisibleColumns {
    pub b_cache_status: bool,
    pub b_message_id: bool,
    pub b_timestamp: bool,
    pub b_play_order: bool,
    pub b_event: bool,
    pub b_asset: bool,
    pub b_actor: bool,
    pub b_category: bool,
}
#[repr(C, align(8))]
pub struct FAudioEventLogSettings {
    pub event_filters: TSet<FString>,
    pub custom_categories_to_events: TMap<FString, FAudioEventLogCustomEvents>,
    pub visible_columns: FAudioEventLogVisibleColumns,
}
#[repr(C, align(8))]
pub struct FSoundDashboardVisibleColumns {
    pub b_mute: bool,
    pub b_solo: bool,
    pub b_plot: bool,
    pub b_name: bool,
    pub b_play_order: bool,
    pub b_priority: bool,
    pub b_distance: bool,
    pub b_distance_attenuation: bool,
    pub b_amplitude: bool,
    pub b_volume: bool,
    pub b_lpf_freq: bool,
    pub b_hpf_freq: bool,
    pub b_pitch: bool,
    pub b_relative_render_cost: bool,
    pub b_actor_label: bool,
    pub b_category: bool,
}
#[repr(C, align(4))]
pub struct FSoundPlotsDashboardPlotRanges {
    pub b_use_custom_amplitude_d_b_range: bool,
    pub amplitude_d_b: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_amplitude_linear_range: bool,
    pub amplitude_linear: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_volume_range: bool,
    pub volume: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_distance_range: bool,
    pub distance: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_distance_attenuation_range: bool,
    pub distance_attenuation: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_pitch_range: bool,
    pub pitch: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_priority_range: bool,
    pub priority: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_lpf_freq_range: bool,
    pub lpf_freq: crate::bindings::core_u_object::FFloatInterval,
    pub b_use_custom_hpf_freq_range: bool,
    pub hpf_freq: crate::bindings::core_u_object::FFloatInterval,
}
#[repr(C, align(8))]
pub struct FSoundDashboardSettings {
    pub amplitude_display_mode: EAudioAmplitudeDisplayMode,
    pub tree_view_mode: ESoundDashboardTreeViewingOptions,
    pub auto_expand_mode: ESoundDashboardAutoExpandOptions,
    pub b_show_stopped_sounds: bool,
    pub stopped_sound_timeout_time: f32,
    pub visible_columns: FSoundDashboardVisibleColumns,
    pub default_plot_ranges: FSoundPlotsDashboardPlotRanges,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioAmplitudeDisplayMode(pub u8);
impl EAudioAmplitudeDisplayMode {
    pub const DECIBELS: EAudioAmplitudeDisplayMode = EAudioAmplitudeDisplayMode(0);
    pub const LINEAR: EAudioAmplitudeDisplayMode = EAudioAmplitudeDisplayMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESoundDashboardTreeViewingOptions(pub u8);
impl ESoundDashboardTreeViewingOptions {
    pub const FULL_TREE: ESoundDashboardTreeViewingOptions = ESoundDashboardTreeViewingOptions(
        0,
    );
    pub const ACTIVE_SOUNDS: ESoundDashboardTreeViewingOptions = ESoundDashboardTreeViewingOptions(
        1,
    );
    pub const FLAT_LIST: ESoundDashboardTreeViewingOptions = ESoundDashboardTreeViewingOptions(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESoundDashboardAutoExpandOptions(pub u8);
impl ESoundDashboardAutoExpandOptions {
    pub const CATEGORIES: ESoundDashboardAutoExpandOptions = ESoundDashboardAutoExpandOptions(
        0,
    );
    pub const EVERYTHING: ESoundDashboardAutoExpandOptions = ESoundDashboardAutoExpandOptions(
        1,
    );
    pub const NOTHING: ESoundDashboardAutoExpandOptions = ESoundDashboardAutoExpandOptions(
        2,
    );
}
