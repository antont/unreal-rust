#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct URewindDebuggerSettings {
    pub camera_mode: ERewindDebuggerCameraMode,
    pub b_should_auto_eject: bool,
    pub b_should_auto_record_on_pie: bool,
    pub playback_rate: f32,
    pub b_show_empty_object_tracks: bool,
    pub hidden_track_types: TArray<FName>,
    pub debug_target_actor: FString,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERewindDebuggerCameraMode(pub u8);
impl ERewindDebuggerCameraMode {
    pub const REPLAY: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(0);
    pub const FOLLOW_TARGET_ACTOR: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(
        1,
    );
    pub const DISABLED: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(2);
}
