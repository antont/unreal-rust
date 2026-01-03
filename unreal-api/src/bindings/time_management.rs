#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTimedDataChannelSampleTime {
    __padding_end: [u8; 24],
}
impl FTimedDataChannelSampleTime {}
#[repr(C, align(4))]
pub struct FTimedDataInputEvaluationData {
    pub distance_to_newest_sample_seconds: f32,
    pub distance_to_oldest_sample_seconds: f32,
}
impl FTimedDataInputEvaluationData {}
pub struct IClockedTimeStep {}
#[repr(C, align(8))]
pub struct UClockedTimeStep {
    __padding_end: [u8; 48],
}
impl UClockedTimeStep {}
#[repr(C, align(8))]
pub struct UFixedFrameRateCustomTimeStep {
    __padding_end: [u8; 48],
}
impl UFixedFrameRateCustomTimeStep {}
#[repr(C, align(8))]
pub struct UCatchupFixedRateCustomTimeStep {
    __padding_end: [u8; 80],
}
impl UCatchupFixedRateCustomTimeStep {}
#[repr(C, align(8))]
pub struct UTimecodeRegressionProvider {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub num_sampled_frames: i32,
    __padding_end: [u8; 180],
}
impl UTimecodeRegressionProvider {}
#[repr(C, align(8))]
pub struct UGenlockedCustomTimeStep {
    __padding_end: [u8; 56],
}
impl UGenlockedCustomTimeStep {}
#[repr(C, align(8))]
pub struct UGenlockedFixedRateCustomTimeStep {
    __padding_end: [u8; 88],
}
impl UGenlockedFixedRateCustomTimeStep {}
#[repr(C, align(8))]
pub struct UGenlockedTimecodeProvider {
    __padding_end: [u8; 96],
}
impl UGenlockedTimecodeProvider {}
#[repr(C, align(8))]
pub struct UMusicalTimeFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UMusicalTimeFunctionLibrary {}
#[repr(C, align(8))]
pub struct UTimeManagementBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTimeManagementBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UTimeSynchronizationSource {
    __padding_end: [u8; 56],
}
impl UTimeSynchronizationSource {}
#[repr(transparent)]
pub struct EFrameNumberDisplayFormats(pub u8);
impl EFrameNumberDisplayFormats {
    pub const NON_DROP_FRAME_TIMECODE: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(
        0,
    );
    pub const DROP_FRAME_TIMECODE: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(
        1,
    );
    pub const SECONDS: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(2);
    pub const FRAMES: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(3);
    pub const CUSTOM: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(4);
    pub const MAX_COUNT: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(5);
}
