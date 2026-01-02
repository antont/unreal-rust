#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UWorldMetricsSubsystem {
    __padding_end: [u8; 10832],
}
impl UWorldMetricsSubsystem {}
#[repr(C, align(8))]
pub struct UWorldMetricInterface {
    __padding_end: [u8; 48],
}
impl UWorldMetricInterface {}
pub struct UWorldMetricsActorTrackerSubscriber {}
pub struct IWorldMetricsActorTrackerSubscriber {}
#[repr(C, align(8))]
pub struct UWorldMetricsExtension {
    __padding_end: [u8; 48],
}
impl UWorldMetricsExtension {}
#[repr(C, align(8))]
pub struct UWorldMetricsActorTracker {
    __padding_end: [u8; 216],
}
impl UWorldMetricsActorTracker {}
