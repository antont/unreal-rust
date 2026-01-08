#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EAudioAmplitudeDisplayMode(pub u8);
impl EAudioAmplitudeDisplayMode {
    pub const DECIBELS: EAudioAmplitudeDisplayMode = EAudioAmplitudeDisplayMode(0);
    pub const LINEAR: EAudioAmplitudeDisplayMode = EAudioAmplitudeDisplayMode(1);
}
#[repr(transparent)]
pub struct ESoundDashboardTreeViewingOptions(pub u8);
impl ESoundDashboardTreeViewingOptions {
    pub const FULL_TREE: ESoundDashboardTreeViewingOptions = ESoundDashboardTreeViewingOptions(
        0,
    );
    pub const ACTIVE_SOUNDS: ESoundDashboardTreeViewingOptions = ESoundDashboardTreeViewingOptions(
        1,
    );
    pub const FLAT_LIST: ESoundDashboardTreeViewingOptions = ESoundDashboardTreeViewingOptions(
        2,
    );
}
#[repr(transparent)]
pub struct ESoundDashboardAutoExpandOptions(pub u8);
impl ESoundDashboardAutoExpandOptions {
    pub const CATEGORIES: ESoundDashboardAutoExpandOptions = ESoundDashboardAutoExpandOptions(
        0,
    );
    pub const EVERYTHING: ESoundDashboardAutoExpandOptions = ESoundDashboardAutoExpandOptions(
        1,
    );
    pub const NOTHING: ESoundDashboardAutoExpandOptions = ESoundDashboardAutoExpandOptions(
        2,
    );
}
