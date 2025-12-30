#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraCacheParams {
    pub cache_parameters: FNiagaraSimCacheCreateParameters,
    pub sim_cache: UPtr<UNiagaraSimCache>,
    pub b_lock_cache_to_read_only: bool,
    pub b_override_quality_level: bool,
    pub record_quality_level: EPerQualityLevels,
    pub cache_replay_play_mode: ENiagaraSimCacheSectionPlayMode,
    pub section_stretch_mode: ENiagaraSimCacheSectionStretchMode,
    pub b_override_record_rate: bool,
    pub cache_record_rate_fps: f32,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraSectionTemplateParameter {
    pub section_range: FMovieSceneFrameRange,
    pub params: FMovieSceneNiagaraCacheParams,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraCacheSectionTemplate {
    pub cache_sections: TArray<FMovieSceneNiagaraSectionTemplateParameter>,
}
pub struct UMovieSceneNiagaraCacheSection {
    pub params: FMovieSceneNiagaraCacheParams,
    pub b_cache_out_of_date: bool,
}
pub struct UMovieSceneNiagaraCacheTrack {
    pub b_is_recording: bool,
    pub animation_sections: TArray<UPtr<UMovieSceneSection>>,
    pub b_cache_recording_enabled: bool,
}
