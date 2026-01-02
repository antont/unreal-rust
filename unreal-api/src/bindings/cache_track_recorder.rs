#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCacheTrackRecorder {
    __padding_end: [u8; 288],
}
impl UCacheTrackRecorder {}
#[repr(transparent)]
pub struct ECacheTrackRecorderState(pub u8);
impl ECacheTrackRecorderState {
    pub const STARTING: ECacheTrackRecorderState = ECacheTrackRecorderState(0);
    pub const PRE_RECORD: ECacheTrackRecorderState = ECacheTrackRecorderState(1);
    pub const TICKING_AFTER_PRE: ECacheTrackRecorderState = ECacheTrackRecorderState(2);
    pub const STARTED: ECacheTrackRecorderState = ECacheTrackRecorderState(3);
    pub const STOPPED: ECacheTrackRecorderState = ECacheTrackRecorderState(4);
    pub const CANCELLED: ECacheTrackRecorderState = ECacheTrackRecorderState(5);
}
