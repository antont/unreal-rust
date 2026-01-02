#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraCacheSection {
    __padding_end: [u8; 528],
}
impl UMovieSceneNiagaraCacheSection {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraCacheTrack {
    __padding_end: [u8; 432],
}
impl UMovieSceneNiagaraCacheTrack {}
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
