#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorFactoryCacheManager {
    __padding_end: [u8; 144],
}
impl UActorFactoryCacheManager {}
#[repr(C, align(8))]
pub struct UCacheCollectionFactory {
    __padding_end: [u8; 136],
}
impl UCacheCollectionFactory {}
#[repr(C, align(8))]
pub struct UMovieSceneChaosCacheTrackRecorder {
    __padding_end: [u8; 184],
}
impl UMovieSceneChaosCacheTrackRecorder {}
#[repr(C, align(8))]
pub struct UTakeRecorderChaosCacheSource {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub chaos_cache_manager: TSoftObjectPtr<
        crate::bindings::chaos_caching::AChaosCacheManager,
    >,
    __padding_end: [u8; 16],
}
impl UTakeRecorderChaosCacheSource {}
