#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTakeRecorderHitchProtectionParameters {
    pub b_enable_hitch_protection: bool,
    pub regression_buffer_size_in_seconds: f32,
    pub custom_timestep: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub max_catchup_seconds: f64,
}
#[repr(C, align(4))]
pub struct FTakeRecorderUserParameters {
    pub b_maximize_viewport: bool,
    pub countdown_seconds: f32,
    pub engine_time_dilation: f32,
    pub b_reset_playhead: bool,
    pub b_stop_at_playback_end: bool,
    pub b_stop_on_rollover: bool,
    pub b_remove_redundant_tracks: bool,
    pub reduce_keys_tolerance: f32,
    pub b_save_recorded_assets: bool,
    pub b_auto_lock: bool,
    pub b_auto_serialize: bool,
}
#[repr(C, align(8))]
pub struct FTakeRecorderProjectParameters {
    pub root_take_save_dir: crate::bindings::core_u_object::FDirectoryPath,
    pub take_save_dir: FString,
    pub sub_sequence_directory: FString,
    pub default_slate: FString,
    pub recording_clock_source: crate::bindings::movie_scene::EUpdateClockSource,
    pub b_start_at_current_timecode: bool,
    pub b_record_timecode: bool,
    pub b_record_sources_into_sub_sequences: bool,
    pub b_record_to_possessable: bool,
    pub default_tracks: TArray<
        crate::bindings::take_track_recorders::FTakeRecorderTrackSettings,
    >,
    pub b_show_notifications: bool,
}
#[repr(C, align(8))]
pub struct FTakeRecorderParameters {
    pub user: FTakeRecorderUserParameters,
    pub project: FTakeRecorderProjectParameters,
    pub take_recorder_mode: ETakeRecorderMode,
    pub start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub b_disable_recording_and_save: bool,
    pub b_open_sequencer: bool,
}
#[repr(C, align(8))]
pub struct FTakeRecorderSequenceParameters {
    pub base_preset: UPtr<crate::bindings::takes_core::UTakePreset>,
    pub base_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub record_into_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub sequence_to_view: UPtr<crate::bindings::level_sequence::ULevelSequence>,
}
#[repr(C, align(8))]
pub struct FTakeRecorderNamingTokensFieldMapping {
    pub field_name: FString,
    pub undefined_keys: TArray<FString>,
}
pub struct UTakeRecorderSubsystemInterface {}
pub struct ITakeRecorderSubsystemInterface {}
pub struct UTakeRecorderHitchVisualizationSettings {
    pub b_show_frame_drop_markers: bool,
    pub b_show_frame_repeat_markers: bool,
    pub b_show_catchup_ranges: bool,
}
pub struct UAssetDefinition_TakePreset {}
pub struct UTakeRecorder {
    pub sequence_asset: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub overlay_widget: UPtr<UTakeRecorderOverlayWidget>,
    pub weak_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
    pub parameters: FTakeRecorderParameters,
}
pub struct UTakeRecorderBlueprintLibrary {}
pub struct UTakeRecorderPanel {}
pub struct UTakeRecorderSubsystem {
    pub implementation: TScriptInterface<ITakeRecorderSubsystemInterface>,
    pub take_recorder_pre_initialize: FTakeRecorderSubsystem_TakeRecorderPreInitialize,
    pub take_recorder_initialized: FTakeRecorderSubsystem_TakeRecorderInitialized,
    pub take_recorder_started: FTakeRecorderSubsystem_TakeRecorderStarted,
    pub take_recorder_stopped: FTakeRecorderSubsystem_TakeRecorderStopped,
    pub take_recorder_finished: FTakeRecorderSubsystem_TakeRecorderFinished,
    pub take_recorder_cancelled: FTakeRecorderSubsystem_TakeRecorderCancelled,
    pub take_recorder_marked_frame_added: FTakeRecorderSubsystem_TakeRecorderMarkedFrameAdded,
    pub take_recorder_slate_changed: FTakeRecorderSubsystem_TakeRecorderSlateChanged,
    pub take_recorder_take_number_changed: FTakeRecorderSubsystem_TakeRecorderTakeNumberChanged,
    pub take_recorder_source_added: FTakeRecorderSubsystem_TakeRecorderSourceAdded,
    pub take_recorder_source_removed: FTakeRecorderSubsystem_TakeRecorderSourceRemoved,
}
pub struct UTakeRecorderSubsystemImplementation {
    pub transient_preset: UPtr<crate::bindings::takes_core::UTakePreset>,
    pub supplied_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub record_into_level_sequence: UPtr<
        crate::bindings::level_sequence::ULevelSequence,
    >,
    pub recording_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub last_recorded_level_sequence: TWeakObjectPtr<
        crate::bindings::level_sequence::ULevelSequence,
    >,
    pub take_meta_data: UPtr<crate::bindings::takes_core::UTakeMetaData>,
    pub transient_take_meta_data: UPtr<crate::bindings::takes_core::UTakeMetaData>,
    pub naming_tokens_data: UPtr<UTakeRecorderNamingTokensData>,
}
pub struct UTakeRecorderOverlayWidget {
    pub recorder: UPtr<UTakeRecorder>,
}
pub struct UTakeRecorderUserSettings {
    pub settings: FTakeRecorderUserParameters,
    pub preset_save_dir: crate::bindings::core_u_object::FDirectoryPath,
    pub last_opened_preset: TSoftObjectPtr<crate::bindings::takes_core::UTakePreset>,
    pub b_is_sequence_open: bool,
    pub b_show_user_settings_on_ui: bool,
}
pub struct UTakeRecorderProjectSettings {
    pub settings: FTakeRecorderProjectParameters,
    pub hitch_protection_settings: FTakeRecorderHitchProtectionParameters,
}
pub struct UTakeRecorderNamingTokensData {
    pub user_defined_tokens: TMap<
        crate::bindings::naming_tokens::FNamingTokenData,
        FText,
    >,
    pub field_to_undefined_keys: TArray<FTakeRecorderNamingTokensFieldMapping>,
    pub evaluated_text_value: FText,
}
pub struct FSetOnTakeRecorderCancelled_OnTakeRecorderCancelled;
pub struct FSetOnTakeRecorderFinished_OnTakeRecorderFinished;
pub struct FSetOnTakeRecorderMarkedFrameAdded_OnTakeRecorderMarkedFrameAdded;
pub struct FSetOnTakeRecorderPanelChanged_OnTakeRecorderPanelChanged;
pub struct FSetOnTakeRecorderPreInitialize_OnTakeRecorderPreInitialize;
pub struct FSetOnTakeRecorderStarted_OnTakeRecorderStarted;
pub struct FSetOnTakeRecorderStopped_OnTakeRecorderStopped;
pub struct FTakeRecorderSubsystem_TakeRecorderPreInitialize;
pub struct FTakeRecorderSubsystem_TakeRecorderInitialized;
pub struct FTakeRecorderSubsystem_TakeRecorderStarted;
pub struct FTakeRecorderSubsystem_TakeRecorderStopped;
pub struct FTakeRecorderSubsystem_TakeRecorderFinished;
pub struct FTakeRecorderSubsystem_TakeRecorderCancelled;
pub struct FTakeRecorderSubsystem_TakeRecorderMarkedFrameAdded;
pub struct FTakeRecorderSubsystem_TakeRecorderSlateChanged;
pub struct FTakeRecorderSubsystem_TakeRecorderTakeNumberChanged;
pub struct FTakeRecorderSubsystem_TakeRecorderSourceAdded;
pub struct FTakeRecorderSubsystem_TakeRecorderSourceRemoved;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETakeRecorderMode(pub u8);
impl ETakeRecorderMode {
    pub const RECORD_NEW_SEQUENCE: ETakeRecorderMode = ETakeRecorderMode(0);
    pub const RECORD_INTO_SEQUENCE: ETakeRecorderMode = ETakeRecorderMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETakeRecorderState(pub u8);
impl ETakeRecorderState {
    pub const PRE_INITIALIZATION: ETakeRecorderState = ETakeRecorderState(0);
    pub const COUNTING_DOWN: ETakeRecorderState = ETakeRecorderState(1);
    pub const PRE_RECORD: ETakeRecorderState = ETakeRecorderState(2);
    pub const TICKING_AFTER_PRE: ETakeRecorderState = ETakeRecorderState(3);
    pub const STARTED: ETakeRecorderState = ETakeRecorderState(4);
    pub const STOPPED: ETakeRecorderState = ETakeRecorderState(5);
    pub const CANCELLED: ETakeRecorderState = ETakeRecorderState(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETakeRecorderPanelMode(pub u8);
impl ETakeRecorderPanelMode {
    pub const NEW_RECORDING: ETakeRecorderPanelMode = ETakeRecorderPanelMode(0);
    pub const RECORDING_INTO: ETakeRecorderPanelMode = ETakeRecorderPanelMode(1);
    pub const EDITING_PRESET: ETakeRecorderPanelMode = ETakeRecorderPanelMode(2);
    pub const REVIEWING_RECORDING: ETakeRecorderPanelMode = ETakeRecorderPanelMode(3);
}
