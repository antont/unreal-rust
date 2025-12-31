#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraCacheParams {
    pub cache_parameters: crate::bindings::niagara::FNiagaraSimCacheCreateParameters,
    pub sim_cache: UPtr<crate::bindings::niagara::UNiagaraSimCache>,
    pub b_lock_cache_to_read_only: bool,
    pub b_override_quality_level: bool,
    pub record_quality_level: crate::bindings::engine::EPerQualityLevels,
    pub cache_replay_play_mode: ENiagaraSimCacheSectionPlayMode,
    pub section_stretch_mode: ENiagaraSimCacheSectionStretchMode,
    pub b_override_record_rate: bool,
    pub cache_record_rate_fps: f32,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraSectionTemplateParameter {
    pub section_range: crate::bindings::movie_scene::FMovieSceneFrameRange,
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
    pub animation_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
    pub b_cache_recording_enabled: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraSimCacheSectionPlayMode(pub u8);
impl ENiagaraSimCacheSectionPlayMode {
    pub const SIM_WITHOUT_CACHE: ENiagaraSimCacheSectionPlayMode = ENiagaraSimCacheSectionPlayMode(
        0,
    );
    pub const DISPLAY_CACHE_ONLY: ENiagaraSimCacheSectionPlayMode = ENiagaraSimCacheSectionPlayMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraSimCacheSectionStretchMode(pub u8);
impl ENiagaraSimCacheSectionStretchMode {
    pub const REPEAT: ENiagaraSimCacheSectionStretchMode = ENiagaraSimCacheSectionStretchMode(
        0,
    );
    pub const TIME_DILATE: ENiagaraSimCacheSectionStretchMode = ENiagaraSimCacheSectionStretchMode(
        1,
    );
}
