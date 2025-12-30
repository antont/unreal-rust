#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UActorFactoryCacheManager {}
pub struct UCacheCollectionFactory {}
pub struct UMovieSceneChaosCacheTrackRecorder {}
pub struct UTakeRecorderChaosCacheSource {
    pub chaos_cache_manager: TSoftObjectPtr<AChaosCacheManager>,
    pub track_recorder: UPtr<UMovieSceneChaosCacheTrackRecorder>,
}
