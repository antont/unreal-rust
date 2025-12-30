#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FGameModeName {
    pub name: FString,
    pub game_mode: FSoftClassPath,
}
#[repr(C, align(8))]
pub struct FTemplateMapInfoOverride {
    pub thumbnail: FSoftObjectPath,
    pub map: FSoftObjectPath,
    pub display_name: FText,
}
#[repr(C, align(8))]
pub struct FAutoCompleteCommand {
    pub command: FString,
    pub desc: FString,
}
pub struct UGameMapsSettings {
    pub editor_startup_map: FSoftObjectPath,
    pub editor_template_map_overrides: TArray<FTemplateMapInfoOverride>,
    pub local_map_options: FString,
    pub transition_map: FSoftObjectPath,
    pub b_use_splitscreen: bool,
    pub two_player_splitscreen_layout: ETwoPlayerSplitScreenType,
    pub three_player_splitscreen_layout: EThreePlayerSplitScreenType,
    pub four_player_splitscreen_layout: EFourPlayerSplitScreenType,
    pub b_show_all_player_widgets_when_splitscreen_disabled: bool,
    pub b_offset_player_gamepad_ids: bool,
    pub game_instance_class: FSoftClassPath,
    pub game_default_map: FSoftObjectPath,
    pub server_default_map: FSoftObjectPath,
    pub global_default_game_mode: FSoftClassPath,
    pub global_default_server_game_mode: FSoftClassPath,
    pub game_mode_map_prefixes: TArray<FGameModeName>,
    pub game_mode_class_aliases: TArray<FGameModeName>,
}
pub struct UGameNetworkManagerSettings {
    pub min_dynamic_bandwidth: i32,
    pub max_dynamic_bandwidth: i32,
    pub total_net_bandwidth: i32,
    pub bad_ping_threshold: i32,
    pub flags_64: u8,
    pub standby_rx_cheat_time: f32,
    pub standby_tx_cheat_time: f32,
    pub percent_missing_for_rx_standby: f32,
    pub percent_missing_for_tx_standby: f32,
    pub percent_for_bad_ping: f32,
    pub join_in_progress_standby_wait_time: f32,
}
pub struct UGameSessionSettings {
    pub max_spectators: i32,
    pub max_players: i32,
    pub flags_56: u8,
}
pub struct UGeneralEngineSettings {}
pub struct UGeneralProjectSettings {
    pub company_name: FString,
    pub company_distinguished_name: FString,
    pub copyright_notice: FString,
    pub description: FString,
    pub homepage: FString,
    pub licensing_terms: FString,
    pub privacy_policy: FString,
    pub project_id: FGuid,
    pub project_name: FString,
    pub project_version: FString,
    pub support_contact: FString,
    pub project_displayed_title: FText,
    pub project_debug_title_info: FText,
    pub b_should_window_preserve_aspect_ratio: bool,
    pub b_use_borderless_window: bool,
    pub b_start_in_vr: bool,
    pub b_allow_window_resize: bool,
    pub b_allow_close: bool,
    pub b_allow_maximize: bool,
    pub b_allow_minimize: bool,
    pub eye_offset_for_fake_stereo_rendering_device: f32,
    pub fov_for_fake_stereo_rendering_device: f32,
    pub top_fov_ratio_for_fake_stereo_rendering_device: f32,
    pub difference_between_eyes_for_fake_stereo_rendering_device: f32,
}
pub struct UHudSettings {
    pub flags_48: u8,
    pub debug_display: TArray<FName>,
}
pub struct UConsoleSettings {
    pub max_scrollback_size: i32,
    pub manual_auto_complete_list: TArray<FAutoCompleteCommand>,
    pub auto_complete_map_paths: TArray<FString>,
    pub background_opacity_percentage: f32,
    pub b_order_top_to_bottom: bool,
    pub b_display_help_in_auto_complete: bool,
    pub input_color: FColor,
    pub history_color: FColor,
    pub auto_complete_command_color: FColor,
    pub auto_complete_c_var_color: FColor,
    pub auto_complete_faded_color: FColor,
}
