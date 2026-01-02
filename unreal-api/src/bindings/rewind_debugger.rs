#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct URewindDebuggerSettings {
    __padding_end: [u8; 152],
}
impl URewindDebuggerSettings {}
#[repr(transparent)]
pub struct ERewindDebuggerCameraMode(pub u8);
impl ERewindDebuggerCameraMode {
    pub const REPLAY: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(0);
    pub const FOLLOW_TARGET_ACTOR: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(
        1,
    );
    pub const DISABLED: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(2);
}
