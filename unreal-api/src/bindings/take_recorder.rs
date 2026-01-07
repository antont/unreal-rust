#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SET_COUNTDOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_GET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_GET_COUNTDOWN_SECONDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_STOP_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_START_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STOPPED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PRE_INITIALIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PANEL_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_MARKED_FRAME_ADDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_CANCELLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_DEFAULT_PARAMETERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_OPEN_TAKE_RECORDER_PANEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_TAKE_RECORDER_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_TAKE_RECORDER_PANEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_DEFAULT_PARAMETERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_ACTIVE_RECORDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_CANCEL_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_STOP_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_START_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SETUP_FOR_VIEWING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_INTO_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_TAKE_PRESET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SETUP_FOR_EDITING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SET_FRAME_RATE_FROM_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_SET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_GET_TAKE_META_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_GET_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_GET_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_GET_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_GET_LAST_RECORDED_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_GET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_CLEAR_PENDING_TAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_PANEL_CAN_START_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_TRY_GET_SEQUENCE_COUNTDOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_STOP_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_START_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_TARGET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_SLATE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_SEQUENCE_COUNTDOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_GLOBAL_RECORD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE_FROM_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_REVIEW_LAST_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_REVERT_CHANGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_RESET_TO_PENDING_TAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_REMOVE_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_REMOVE_ACTOR_FROM_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_MARK_FRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_IS_REVIEWING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_IS_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_RECORDER_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_META_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_RECORD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_SLATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_PENDING_TAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_NUMBER_OF_TAKES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_NEXT_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_LAST_RECORDED_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_GLOBAL_RECORD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_GET_ALL_SOURCES_COPY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_CLEAR_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_CLEAR_PENDING_TAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_CAN_REVIEW_LAST_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_CANCEL_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE_FOR_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorder::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCountdown"),
            &raw mut U_TAKE_RECORDER_SET_COUNTDOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut U_TAKE_RECORDER_GET_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_TAKE_RECORDER_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCountdownSeconds"),
            &raw mut U_TAKE_RECORDER_GET_COUNTDOWN_SECONDS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorderBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopRecording"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_STOP_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecording"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_START_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderStopped"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STOPPED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderStarted"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderPreInitialize"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PRE_INITIALIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderPanelChanged"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PANEL_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderMarkedFrameAdded"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_MARKED_FRAME_ADDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderFinished"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderCancelled"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_CANCELLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultParameters"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_DEFAULT_PARAMETERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenTakeRecorderPanel"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_OPEN_TAKE_RECORDER_PANEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTakeRecorderEnabled"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_TAKE_RECORDER_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecording"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTakeRecorderPanel"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_TAKE_RECORDER_PANEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultParameters"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_DEFAULT_PARAMETERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveRecorder"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_ACTIVE_RECORDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelRecording"),
            &raw mut U_TAKE_RECORDER_BLUEPRINT_LIBRARY_CANCEL_RECORDING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorderPanel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopRecording"),
            &raw mut U_TAKE_RECORDER_PANEL_STOP_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecording"),
            &raw mut U_TAKE_RECORDER_PANEL_START_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupForViewing"),
            &raw mut U_TAKE_RECORDER_PANEL_SETUP_FOR_VIEWING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupForRecordingInto_LevelSequence"),
            &raw mut U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_INTO_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupForRecording_TakePreset"),
            &raw mut U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_TAKE_PRESET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupForRecording_LevelSequence"),
            &raw mut U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupForEditing"),
            &raw mut U_TAKE_RECORDER_PANEL_SETUP_FOR_EDITING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRateFromTimecode"),
            &raw mut U_TAKE_RECORDER_PANEL_SET_FRAME_RATE_FROM_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRate"),
            &raw mut U_TAKE_RECORDER_PANEL_SET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTakeMetaData"),
            &raw mut U_TAKE_RECORDER_PANEL_GET_TAKE_META_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSources"),
            &raw mut U_TAKE_RECORDER_PANEL_GET_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMode"),
            &raw mut U_TAKE_RECORDER_PANEL_GET_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelSequence"),
            &raw mut U_TAKE_RECORDER_PANEL_GET_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastRecordedLevelSequence"),
            &raw mut U_TAKE_RECORDER_PANEL_GET_LAST_RECORDED_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRate"),
            &raw mut U_TAKE_RECORDER_PANEL_GET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearPendingTake"),
            &raw mut U_TAKE_RECORDER_PANEL_CLEAR_PENDING_TAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanStartRecording"),
            &raw mut U_TAKE_RECORDER_PANEL_CAN_START_RECORDING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorderSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryGetSequenceCountdown"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_TRY_GET_SEQUENCE_COUNTDOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopRecording"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_STOP_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecording"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_START_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetSequence"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_TARGET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTakeNumber"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_TAKE_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSlateName"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_SLATE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequenceCountdown"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_SEQUENCE_COUNTDOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGlobalRecordSettings"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_GLOBAL_RECORD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRateFromTimecode"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE_FROM_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRate"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReviewLastRecording"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_REVIEW_LAST_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RevertChanges"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_REVERT_CHANGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetToPendingTake"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_RESET_TO_PENDING_TAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSource"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_REMOVE_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorFromSources"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_REMOVE_ACTOR_FROM_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkFrame"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_MARK_FRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReviewing"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_IS_REVIEWING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecording"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_IS_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTakeRecorderMode"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_RECORDER_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTakeMetaData"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_META_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSources"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceRecordSettings"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_RECORD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceByClass"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceActor"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlates"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_SLATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPendingTake"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_PENDING_TAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfTakes"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_NUMBER_OF_TAKES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNextTakeNumber"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_NEXT_TAKE_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelSequence"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastRecordedLevelSequence"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_LAST_RECORDED_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalRecordSettings"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_GLOBAL_RECORD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRate"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllSourcesCopy"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_GET_ALL_SOURCES_COPY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSources"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_CLEAR_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearPendingTake"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_CLEAR_PENDING_TAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanReviewLastRecording"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_CAN_REVIEW_LAST_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelRecording"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_CANCEL_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSourceForActor"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE_FOR_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSource"),
            &raw mut U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE,
        );
    }
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
pub struct ITakeRecorderSubsystemInterface {}
#[repr(C, align(8))]
pub struct UTakeRecorderSubsystemInterface {
    __padding_end: [u8; 48],
}
impl UTakeRecorderSubsystemInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSubsystemInterface")
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
pub struct UTakeRecorderHitchVisualizationSettings {
    __padding_end: [u8; 56],
}
impl UTakeRecorderHitchVisualizationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderHitchVisualizationSettings")
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
pub struct UAssetDefinition_TakePreset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_TakePreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_TakePreset")
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
pub struct UTakeRecorder {
    __padding_end: [u8; 624],
}
impl UTakeRecorder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorder")
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
    pub fn set_countdown(&mut self, in_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SET_COUNTDOWN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SET_COUNTDOWN,
                __buffer,
            )
        };
    }
    pub fn get_state(&self) -> ETakeRecorderState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_GET_STATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_GET_STATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ETakeRecorderState>().read() }
    }
    pub fn get_sequence(&self) -> UPtr<crate::bindings::level_sequence::ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_GET_SEQUENCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_GET_SEQUENCE,
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
    pub fn get_countdown_seconds(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_GET_COUNTDOWN_SECONDS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_GET_COUNTDOWN_SECONDS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTakeRecorderBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderBlueprintLibrary")
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
    pub fn stop_recording() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_STOP_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_STOP_RECORDING,
                __buffer,
            )
        };
    }
    pub fn start_recording(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        sources: UPtr<crate::bindings::takes_core::UTakeRecorderSources>,
        meta_data: UPtr<crate::bindings::takes_core::UTakeMetaData>,
        parameters: &FTakeRecorderParameters,
    ) -> UPtr<UTakeRecorder> {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_START_RECORDING,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sources,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSources>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeMetaData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(24).cast::<FTakeRecorderParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_START_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<UPtr<UTakeRecorder>>().read() }
    }
    pub fn set_on_take_recorder_stopped(
        on_take_recorder_stopped: FSetOnTakeRecorderStopped_OnTakeRecorderStopped,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STOPPED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_stopped,
                __buffer
                    .add(0)
                    .cast::<FSetOnTakeRecorderStopped_OnTakeRecorderStopped>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STOPPED,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_started(
        on_take_recorder_started: FSetOnTakeRecorderStarted_OnTakeRecorderStarted,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STARTED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_started,
                __buffer
                    .add(0)
                    .cast::<FSetOnTakeRecorderStarted_OnTakeRecorderStarted>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_STARTED,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_pre_initialize(
        on_take_recorder_pre_initialize: FSetOnTakeRecorderPreInitialize_OnTakeRecorderPreInitialize,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PRE_INITIALIZE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_pre_initialize,
                __buffer
                    .add(0)
                    .cast::<
                        FSetOnTakeRecorderPreInitialize_OnTakeRecorderPreInitialize,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PRE_INITIALIZE,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_panel_changed(
        on_take_recorder_panel_changed: FSetOnTakeRecorderPanelChanged_OnTakeRecorderPanelChanged,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PANEL_CHANGED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_panel_changed,
                __buffer
                    .add(0)
                    .cast::<FSetOnTakeRecorderPanelChanged_OnTakeRecorderPanelChanged>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_PANEL_CHANGED,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_marked_frame_added(
        on_take_recorder_marked_frame_added: FSetOnTakeRecorderMarkedFrameAdded_OnTakeRecorderMarkedFrameAdded,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_MARKED_FRAME_ADDED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_marked_frame_added,
                __buffer
                    .add(0)
                    .cast::<
                        FSetOnTakeRecorderMarkedFrameAdded_OnTakeRecorderMarkedFrameAdded,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_MARKED_FRAME_ADDED,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_finished(
        on_take_recorder_finished: FSetOnTakeRecorderFinished_OnTakeRecorderFinished,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_FINISHED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_finished,
                __buffer
                    .add(0)
                    .cast::<FSetOnTakeRecorderFinished_OnTakeRecorderFinished>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_FINISHED,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_cancelled(
        on_take_recorder_cancelled: FSetOnTakeRecorderCancelled_OnTakeRecorderCancelled,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_CANCELLED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_cancelled,
                __buffer
                    .add(0)
                    .cast::<FSetOnTakeRecorderCancelled_OnTakeRecorderCancelled>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_CANCELLED,
                __buffer,
            )
        };
    }
    pub fn set_default_parameters(default_parameters: &FTakeRecorderParameters) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_DEFAULT_PARAMETERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                default_parameters,
                __buffer.add(0).cast::<FTakeRecorderParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_SET_DEFAULT_PARAMETERS,
                __buffer,
            )
        };
    }
    pub fn open_take_recorder_panel() -> UPtr<UTakeRecorderPanel> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_OPEN_TAKE_RECORDER_PANEL,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_OPEN_TAKE_RECORDER_PANEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTakeRecorderPanel>>().read() }
    }
    pub fn is_take_recorder_enabled() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_TAKE_RECORDER_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_TAKE_RECORDER_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_recording() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_IS_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_take_recorder_panel() -> UPtr<UTakeRecorderPanel> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_TAKE_RECORDER_PANEL,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_TAKE_RECORDER_PANEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTakeRecorderPanel>>().read() }
    }
    pub fn get_default_parameters() -> FTakeRecorderParameters {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_DEFAULT_PARAMETERS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_DEFAULT_PARAMETERS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FTakeRecorderParameters>().read() }
    }
    pub fn get_active_recorder() -> UPtr<UTakeRecorder> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_ACTIVE_RECORDER,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_GET_ACTIVE_RECORDER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTakeRecorder>>().read() }
    }
    pub fn cancel_recording() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_CANCEL_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::take_recorder::UTakeRecorderBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_BLUEPRINT_LIBRARY_CANCEL_RECORDING,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderPanel {
    __padding_end: [u8; 64],
}
impl UTakeRecorderPanel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderPanel")
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
    pub fn stop_recording(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_STOP_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_STOP_RECORDING,
                __buffer,
            )
        };
    }
    pub fn start_recording(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_START_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_START_RECORDING,
                __buffer,
            )
        };
    }
    pub fn setup_for_viewing(
        &mut self,
        level_sequence_asset: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_VIEWING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence_asset,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_VIEWING,
                __buffer,
            )
        };
    }
    pub fn setup_for_recording_into_level_sequence(
        &mut self,
        level_sequence_asset: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_INTO_LEVEL_SEQUENCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence_asset,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_INTO_LEVEL_SEQUENCE,
                __buffer,
            )
        };
    }
    pub fn setup_for_recording_take_preset(
        &mut self,
        take_preset_asset: UPtr<crate::bindings::takes_core::UTakePreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_TAKE_PRESET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &take_preset_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::takes_core::UTakePreset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_TAKE_PRESET,
                __buffer,
            )
        };
    }
    pub fn setup_for_recording_level_sequence(
        &mut self,
        level_sequence_asset: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_LEVEL_SEQUENCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence_asset,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_RECORDING_LEVEL_SEQUENCE,
                __buffer,
            )
        };
    }
    pub fn setup_for_editing(
        &mut self,
        take_preset: UPtr<crate::bindings::takes_core::UTakePreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_EDITING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &take_preset,
                __buffer.add(0).cast::<UPtr<crate::bindings::takes_core::UTakePreset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SETUP_FOR_EDITING,
                __buffer,
            )
        };
    }
    pub fn set_frame_rate_from_timecode(&mut self, b_in_from_timecode: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_from_timecode,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
    }
    pub fn set_frame_rate(
        &mut self,
        in_frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SET_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_rate,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_SET_FRAME_RATE,
                __buffer,
            )
        };
    }
    pub fn get_take_meta_data(
        &self,
    ) -> UPtr<crate::bindings::takes_core::UTakeMetaData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_TAKE_META_DATA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_TAKE_META_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::takes_core::UTakeMetaData>>()
                .read()
        }
    }
    pub fn get_sources(
        &self,
    ) -> UPtr<crate::bindings::takes_core::UTakeRecorderSources> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_SOURCES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_SOURCES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSources>>()
                .read()
        }
    }
    pub fn get_mode(&self) -> ETakeRecorderPanelMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_MODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_MODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ETakeRecorderPanelMode>().read() }
    }
    pub fn get_level_sequence(
        &self,
    ) -> UPtr<crate::bindings::level_sequence::ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_LEVEL_SEQUENCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_LEVEL_SEQUENCE,
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
    pub fn get_last_recorded_level_sequence(
        &self,
    ) -> UPtr<crate::bindings::level_sequence::ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_LAST_RECORDED_LEVEL_SEQUENCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_LAST_RECORDED_LEVEL_SEQUENCE,
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
    pub fn get_frame_rate(&self) -> crate::bindings::core_u_object::FFrameRate {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_FRAME_RATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_GET_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>().read()
        }
    }
    pub fn clear_pending_take(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_CLEAR_PENDING_TAKE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_CLEAR_PENDING_TAKE,
                __buffer,
            )
        };
    }
    pub fn can_start_recording(&self, out_error_text: &mut FText) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_CAN_START_RECORDING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error_text,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_PANEL_CAN_START_RECORDING,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FText>().swap(out_error_text);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderSubsystem {
    __padding_end: [u8; 600],
}
impl UTakeRecorderSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSubsystem")
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
    pub fn try_get_sequence_countdown(&self, out_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_TRY_GET_SEQUENCE_COUNTDOWN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_TRY_GET_SEQUENCE_COUNTDOWN,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(out_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn stop_recording(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_STOP_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_STOP_RECORDING,
                __buffer,
            )
        };
    }
    pub fn start_recording(
        &mut self,
        b_open_sequencer: bool,
        b_show_error_message: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_START_RECORDING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_open_sequencer,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_error_message,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_START_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn set_target_sequence(&mut self, in_data: &FTakeRecorderSequenceParameters) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_TARGET_SEQUENCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_data,
                __buffer.add(0).cast::<FTakeRecorderSequenceParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_TARGET_SEQUENCE,
                __buffer,
            )
        };
    }
    pub fn set_take_number(&mut self, in_new_take_number: i32, b_emit_changed: bool) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_take_number,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_emit_changed,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_TAKE_NUMBER,
                __buffer,
            )
        };
    }
    pub fn set_slate_name(&mut self, in_slate_name: FString, b_emit_changed: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_SLATE_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_slate_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_emit_changed,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_SLATE_NAME,
                __buffer,
            )
        };
    }
    pub fn set_sequence_countdown(&mut self, in_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_SEQUENCE_COUNTDOWN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_SEQUENCE_COUNTDOWN,
                __buffer,
            )
        };
    }
    pub fn set_global_record_settings(
        &mut self,
        in_parameters: &FTakeRecorderParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_GLOBAL_RECORD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameters,
                __buffer.add(0).cast::<FTakeRecorderParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_GLOBAL_RECORD_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn set_frame_rate_from_timecode(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
    }
    pub fn set_frame_rate(
        &mut self,
        in_frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_rate,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_SET_FRAME_RATE,
                __buffer,
            )
        };
    }
    pub fn review_last_recording(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REVIEW_LAST_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REVIEW_LAST_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn revert_changes(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REVERT_CHANGES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REVERT_CHANGES,
                __buffer,
            )
        };
    }
    pub fn reset_to_pending_take(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_RESET_TO_PENDING_TAKE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_RESET_TO_PENDING_TAKE,
                __buffer,
            )
        };
    }
    pub fn remove_source(
        &mut self,
        in_source: UPtr<crate::bindings::takes_core::UTakeRecorderSource>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REMOVE_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REMOVE_SOURCE,
                __buffer,
            )
        };
    }
    pub fn remove_actor_from_sources(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REMOVE_ACTOR_FROM_SOURCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_REMOVE_ACTOR_FROM_SOURCES,
                __buffer,
            )
        };
    }
    pub fn mark_frame(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_MARK_FRAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_MARK_FRAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_reviewing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_IS_REVIEWING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_IS_REVIEWING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_recording(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_IS_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_IS_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_take_recorder_mode(&self) -> ETakeRecorderMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_RECORDER_MODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_RECORDER_MODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ETakeRecorderMode>().read() }
    }
    pub fn get_take_meta_data(
        &self,
    ) -> UPtr<crate::bindings::takes_core::UTakeMetaData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_META_DATA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_TAKE_META_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::takes_core::UTakeMetaData>>()
                .read()
        }
    }
    pub fn get_state(&self) -> ETakeRecorderState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_STATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_STATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ETakeRecorderState>().read() }
    }
    pub fn get_sources(
        &self,
    ) -> UPtr<crate::bindings::takes_core::UTakeRecorderSources> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSources>>()
                .read()
        }
    }
    pub fn get_source_record_settings(
        &self,
        in_source: UPtr<crate::bindings::takes_core::UTakeRecorderSource>,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_RECORD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_RECORD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_source_by_class(
        &self,
        in_source_class: TSubclassOf<crate::bindings::takes_core::UTakeRecorderSource>,
    ) -> UPtr<crate::bindings::takes_core::UTakeRecorderSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_BY_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::takes_core::UTakeRecorderSource>,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_BY_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>()
                .read()
        }
    }
    pub fn get_source_actor(
        &self,
        in_source: UPtr<crate::bindings::takes_core::UTakeRecorderSource>,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SOURCE_ACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn get_slates(
        &self,
        in_package_path: FName,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SLATES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_package_path,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_SLATES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
    pub fn get_pending_take(&self) -> UPtr<crate::bindings::takes_core::UTakePreset> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_PENDING_TAKE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_PENDING_TAKE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::takes_core::UTakePreset>>()
                .read()
        }
    }
    pub fn get_number_of_takes(
        &self,
        in_slate: FString,
        out_max_take: &mut i32,
        out_num_takes: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_NUMBER_OF_TAKES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_slate,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_max_take,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_num_takes,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_NUMBER_OF_TAKES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(out_max_take);
        }
        unsafe {
            __buffer.add(20).cast::<i32>().swap(out_num_takes);
        }
    }
    pub fn get_next_take_number(&self, in_slate: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_NEXT_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_slate,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_NEXT_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_level_sequence(
        &self,
    ) -> UPtr<crate::bindings::level_sequence::ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_LEVEL_SEQUENCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_LEVEL_SEQUENCE,
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
    pub fn get_last_recorded_level_sequence(
        &self,
    ) -> UPtr<crate::bindings::level_sequence::ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_LAST_RECORDED_LEVEL_SEQUENCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_LAST_RECORDED_LEVEL_SEQUENCE,
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
    pub fn get_global_record_settings(&self) -> FTakeRecorderParameters {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_GLOBAL_RECORD_SETTINGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_GLOBAL_RECORD_SETTINGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FTakeRecorderParameters>().read() }
    }
    pub fn get_frame_rate(&self) -> crate::bindings::core_u_object::FFrameRate {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_FRAME_RATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>().read()
        }
    }
    pub fn get_all_sources_copy(
        &self,
    ) -> TArray<UPtr<crate::bindings::takes_core::UTakeRecorderSource>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_ALL_SOURCES_COPY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_GET_ALL_SOURCES_COPY,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>>()
                .read()
        }
    }
    pub fn clear_sources(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CLEAR_SOURCES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CLEAR_SOURCES,
                __buffer,
            )
        };
    }
    pub fn clear_pending_take(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CLEAR_PENDING_TAKE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CLEAR_PENDING_TAKE,
                __buffer,
            )
        };
    }
    pub fn can_review_last_recording(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CAN_REVIEW_LAST_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CAN_REVIEW_LAST_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn cancel_recording(&mut self, b_show_confirm_message: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CANCEL_RECORDING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_confirm_message,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_CANCEL_RECORDING,
                __buffer,
            )
        };
    }
    pub fn add_source_for_actor(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        b_reduce_keys: bool,
        b_show_progress: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE_FOR_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reduce_keys,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_progress,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE_FOR_ACTOR,
                __buffer,
            )
        };
    }
    pub fn add_source(
        &mut self,
        in_source_class: TSubclassOf<crate::bindings::takes_core::UTakeRecorderSource>,
    ) -> UPtr<crate::bindings::takes_core::UTakeRecorderSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::takes_core::UTakeRecorderSource>,
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
                crate::bindings::take_recorder::U_TAKE_RECORDER_SUBSYSTEM_ADD_SOURCE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderSubsystemImplementation {
    __padding_end: [u8; 224],
}
impl UTakeRecorderSubsystemImplementation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSubsystemImplementation")
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
pub struct UTakeRecorderOverlayWidget {
    #[doc(hidden)]
    __padding_1288: [u8; 1288],
    pub recorder: UPtr<UTakeRecorder>,
}
impl UTakeRecorderOverlayWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderOverlayWidget")
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
pub struct UTakeRecorderUserSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub settings: FTakeRecorderUserParameters,
    __padding_end: [u8; 72],
}
impl UTakeRecorderUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderUserSettings")
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
pub struct UTakeRecorderProjectSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub settings: FTakeRecorderProjectParameters,
    __padding_end: [u8; 80],
}
impl UTakeRecorderProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderProjectSettings")
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
pub struct UTakeRecorderNamingTokensData {
    __padding_end: [u8; 264],
}
impl UTakeRecorderNamingTokensData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNamingTokensData")
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
