#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FLKFSResults {
    pub channel: i32,
    pub timestamp: f32,
    pub energy: f32,
    pub loudness: f32,
    pub short_term_loudness: f32,
    pub integrated_loudness: f32,
    pub gated_loudness: f32,
}
#[repr(C, align(4))]
pub struct FLKFSNRTResults {
    pub channel: i32,
    pub timestamp: f32,
    pub energy: f32,
    pub loudness: f32,
    pub short_term_loudness: f32,
}
#[repr(C, align(4))]
pub struct FLKFSNRTAggregateStats {
    pub integrated_loudness: f32,
    pub gated_loudness: f32,
}
#[repr(C, align(8))]
pub struct FLKFSNRTChannelData {
    pub aggregate_stats: FLKFSNRTAggregateStats,
    pub loudness_array: TArray<FLKFSNRTResults>,
    pub loudness_interval: FFloatInterval,
}
#[repr(C, align(8))]
pub struct FLKFSNRTWaveData {
    pub b_is_sorted_chronologically: bool,
    pub duration_in_seconds: f32,
    pub channel_data: TMap<i32, FLKFSNRTChannelData>,
}
