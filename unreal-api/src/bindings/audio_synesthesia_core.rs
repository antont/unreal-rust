#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
impl FLKFSResults {}
#[repr(C, align(4))]
pub struct FLKFSNRTResults {
    pub channel: i32,
    pub timestamp: f32,
    pub energy: f32,
    pub loudness: f32,
    pub short_term_loudness: f32,
}
impl FLKFSNRTResults {}
#[repr(C, align(4))]
pub struct FLKFSNRTAggregateStats {
    pub integrated_loudness: f32,
    pub gated_loudness: f32,
}
impl FLKFSNRTAggregateStats {}
