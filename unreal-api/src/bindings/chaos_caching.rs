#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FObservedComponent {
    pub cache_name: FName,
    #[doc(hidden)]
    __padding_64: [u8; 48],
    pub soft_component_ref: crate::bindings::engine::FSoftComponentReference,
    pub b_is_simulating: bool,
    pub b_playback_enabled: bool,
    pub usd_cache_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub b_has_notify_breaks: bool,
    __padding_end: [u8; 255],
}
impl FObservedComponent {}
#[repr(C, align(8))]
pub struct UChaosCacheCollection {
    __padding_end: [u8; 72],
}
impl UChaosCacheCollection {}
#[repr(C, align(8))]
pub struct AChaosCacheManager {
    #[doc(hidden)]
    __padding_1148: [u8; 1148],
    pub start_time: f32,
    __padding_end: [u8; 176],
}
impl AChaosCacheManager {}
#[repr(C, align(8))]
pub struct AChaosCachePlayer {
    __padding_end: [u8; 1328],
}
impl AChaosCachePlayer {}
pub struct UChaosCacheData {}
pub struct IChaosCacheData {}
#[repr(C, align(16))]
pub struct UChaosCache {
    __padding_end: [u8; 912],
}
impl UChaosCache {}
#[repr(C, align(8))]
pub struct UMovieSceneChaosCacheSection {
    __padding_end: [u8; 416],
}
impl UMovieSceneChaosCacheSection {}
#[repr(C, align(8))]
pub struct UMovieSceneChaosCacheTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneChaosCacheTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnableChaosCacheBinding {
    __padding_end: [u8; 80],
}
impl UMovieSceneSpawnableChaosCacheBinding {}
#[repr(transparent)]
pub struct EChaosCacheInterpolationMode(pub u8);
impl EChaosCacheInterpolationMode {
    pub const QUAT_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        0,
    );
    pub const EULER_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        1,
    );
    pub const DUAL_QUAT_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ECacheMode(pub u8);
impl ECacheMode {
    pub const NONE: ECacheMode = ECacheMode(0);
    pub const PLAY: ECacheMode = ECacheMode(1);
    pub const RECORD: ECacheMode = ECacheMode(2);
}
#[repr(transparent)]
pub struct EStartMode(pub u8);
impl EStartMode {
    pub const TIMED: EStartMode = EStartMode(0);
    pub const TRIGGERED: EStartMode = EStartMode(1);
}
