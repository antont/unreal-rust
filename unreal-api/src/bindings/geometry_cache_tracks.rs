#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheParams {
    pub geometry_cache_asset: UPtr<UGeometryCache>,
    pub first_loop_start_frame_offset: FFrameNumber,
    pub start_frame_offset: FFrameNumber,
    pub end_frame_offset: FFrameNumber,
    pub play_rate: f32,
    pub flags_24: u8,
    pub start_offset_deprecated: f32,
    pub end_offset_deprecated: f32,
    pub geometry_cache_deprecated: FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheSectionTemplateParameters {
    pub section_start_time: FFrameNumber,
    pub section_end_time: FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheSectionTemplate {
    pub params: FMovieSceneGeometryCacheSectionTemplateParameters,
}
pub struct UMovieSceneGeometryCacheSection {
    pub params: FMovieSceneGeometryCacheParams,
}
pub struct UMovieSceneGeometryCacheTrack {
    pub animation_sections: TArray<UPtr<UMovieSceneSection>>,
}
