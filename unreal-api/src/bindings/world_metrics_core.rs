#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FWorldMetricCollection {
    pub metrics: TArray<UPtr<UWorldMetricInterface>>,
    pub subsystem: TWeakObjectPtr<UWorldMetricsSubsystem>,
    pub b_is_enabled: bool,
}
pub struct UWorldMetricsSubsystem {
    pub metrics: TArray<UPtr<UWorldMetricInterface>>,
    pub update_rate_in_seconds: f32,
    pub warm_up_frames: i32,
}
pub struct UWorldMetricInterface {}
pub struct UWorldMetricsActorTrackerSubscriber {}
pub struct IWorldMetricsActorTrackerSubscriber {}
pub struct UWorldMetricsExtension {}
pub struct UWorldMetricsActorTracker {}
