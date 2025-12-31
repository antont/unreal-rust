#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UActorFactoryCacheManager {}
pub struct UCacheCollectionFactory {}
pub struct UMovieSceneChaosCacheTrackRecorder {}
pub struct UTakeRecorderChaosCacheSource {
    pub chaos_cache_manager: TSoftObjectPtr<
        crate::bindings::chaos_caching::AChaosCacheManager,
    >,
    pub track_recorder: UPtr<UMovieSceneChaosCacheTrackRecorder>,
}
