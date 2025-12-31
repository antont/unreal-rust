#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheParams {
    pub geometry_cache_asset: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub play_rate: f32,
    pub flags_24: u8,
    pub start_offset_deprecated: f32,
    pub end_offset_deprecated: f32,
    pub geometry_cache_deprecated: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheSectionTemplateParameters {
    pub section_start_time: crate::bindings::core_u_object::FFrameNumber,
    pub section_end_time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheSectionTemplate {
    pub params: FMovieSceneGeometryCacheSectionTemplateParameters,
}
pub struct UMovieSceneGeometryCacheSection {
    pub params: FMovieSceneGeometryCacheParams,
}
pub struct UMovieSceneGeometryCacheTrack {
    pub animation_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
