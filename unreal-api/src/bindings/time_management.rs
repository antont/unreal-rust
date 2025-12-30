#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FTimedDataChannelSampleTime {}
#[repr(C, align(4))]
pub struct FTimedDataInputEvaluationData {
    pub distance_to_newest_sample_seconds: f32,
    pub distance_to_oldest_sample_seconds: f32,
}
pub struct UClockedTimeStep {}
pub struct IClockedTimeStep {}
pub struct UFixedFrameRateCustomTimeStep {}
pub struct UCatchupFixedRateCustomTimeStep {
    pub frame_rate: FFrameRate,
    pub max_catchup_seconds: f64,
}
pub struct UTimecodeRegressionProvider {
    pub num_sampled_frames: i32,
    pub owning_engine: UPtr<UEngine>,
    pub timecode_impl: UPtr<UTimecodeProvider>,
}
pub struct UGenlockedCustomTimeStep {
    pub b_auto_detect_format: bool,
    pub b_wait_for_both_fields: bool,
}
pub struct UGenlockedFixedRateCustomTimeStep {
    pub frame_rate: FFrameRate,
    pub b_should_block: bool,
    pub b_force_single_frame_delta_time: bool,
}
pub struct UGenlockedTimecodeProvider {
    pub b_use_genlock_to_count: bool,
}
pub struct UMusicalTimeFunctionLibrary {}
pub struct UTimeManagementBlueprintLibrary {}
pub struct UTimeSynchronizationSource {
    pub b_use_for_synchronization: bool,
    pub frame_offset: i32,
}
