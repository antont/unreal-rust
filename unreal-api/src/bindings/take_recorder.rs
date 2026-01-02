#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    __padding_end: [u8; 1],
}
impl FTakeRecorderUserParameters {}
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
    #[doc(hidden)]
    __padding_88: [u8; 19],
    pub b_show_notifications: bool,
    __padding_end: [u8; 7],
}
impl FTakeRecorderProjectParameters {}
#[repr(C, align(8))]
pub struct FTakeRecorderParameters {
    pub user: FTakeRecorderUserParameters,
    pub project: FTakeRecorderProjectParameters,
    pub take_recorder_mode: ETakeRecorderMode,
    pub start_frame: crate::bindings::core_u_object::FFrameNumber,
    #[doc(hidden)]
    __padding_129: [u8; 1],
    pub b_open_sequencer: bool,
    __padding_end: [u8; 6],
}
impl FTakeRecorderParameters {}
#[repr(C, align(8))]
pub struct FTakeRecorderSequenceParameters {
    pub base_preset: UPtr<crate::bindings::takes_core::UTakePreset>,
    pub base_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub record_into_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub sequence_to_view: UPtr<crate::bindings::level_sequence::ULevelSequence>,
}
impl FTakeRecorderSequenceParameters {}
pub struct UTakeRecorderSubsystemInterface {}
pub struct ITakeRecorderSubsystemInterface {}
#[repr(C, align(8))]
pub struct UTakeRecorderHitchVisualizationSettings {
    __padding_end: [u8; 56],
}
impl UTakeRecorderHitchVisualizationSettings {}
#[repr(C, align(8))]
pub struct UAssetDefinition_TakePreset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_TakePreset {}
#[repr(C, align(8))]
pub struct UTakeRecorder {
    __padding_end: [u8; 624],
}
impl UTakeRecorder {}
#[repr(C, align(8))]
pub struct UTakeRecorderBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTakeRecorderBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UTakeRecorderPanel {
    __padding_end: [u8; 64],
}
impl UTakeRecorderPanel {}
#[repr(C, align(8))]
pub struct UTakeRecorderSubsystem {
    __padding_end: [u8; 600],
}
impl UTakeRecorderSubsystem {}
#[repr(C, align(8))]
pub struct UTakeRecorderSubsystemImplementation {
    __padding_end: [u8; 224],
}
impl UTakeRecorderSubsystemImplementation {}
#[repr(C, align(8))]
pub struct UTakeRecorderOverlayWidget {
    #[doc(hidden)]
    __padding_1288: [u8; 1288],
    pub recorder: UPtr<UTakeRecorder>,
}
impl UTakeRecorderOverlayWidget {}
#[repr(C, align(8))]
pub struct UTakeRecorderUserSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub settings: FTakeRecorderUserParameters,
    __padding_end: [u8; 72],
}
impl UTakeRecorderUserSettings {}
#[repr(C, align(8))]
pub struct UTakeRecorderProjectSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub settings: FTakeRecorderProjectParameters,
    __padding_end: [u8; 80],
}
impl UTakeRecorderProjectSettings {}
#[repr(C, align(8))]
pub struct UTakeRecorderNamingTokensData {
    __padding_end: [u8; 264],
}
impl UTakeRecorderNamingTokensData {}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderCancelled_OnTakeRecorderCancelled {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderFinished_OnTakeRecorderFinished {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderMarkedFrameAdded_OnTakeRecorderMarkedFrameAdded {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderPanelChanged_OnTakeRecorderPanelChanged {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderPreInitialize_OnTakeRecorderPreInitialize {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderStarted_OnTakeRecorderStarted {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderStopped_OnTakeRecorderStopped {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderPreInitialize {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderInitialized {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderStarted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderStopped {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderMarkedFrameAdded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderSlateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderTakeNumberChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderSourceAdded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTakeRecorderSubsystem_TakeRecorderSourceRemoved {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ETakeRecorderMode(pub u8);
impl ETakeRecorderMode {
    pub const RECORD_NEW_SEQUENCE: ETakeRecorderMode = ETakeRecorderMode(0);
    pub const RECORD_INTO_SEQUENCE: ETakeRecorderMode = ETakeRecorderMode(1);
}
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
#[repr(transparent)]
pub struct ETakeRecorderPanelMode(pub u8);
impl ETakeRecorderPanelMode {
    pub const NEW_RECORDING: ETakeRecorderPanelMode = ETakeRecorderPanelMode(0);
    pub const RECORDING_INTO: ETakeRecorderPanelMode = ETakeRecorderPanelMode(1);
    pub const EDITING_PRESET: ETakeRecorderPanelMode = ETakeRecorderPanelMode(2);
    pub const REVIEWING_RECORDING: ETakeRecorderPanelMode = ETakeRecorderPanelMode(3);
}
