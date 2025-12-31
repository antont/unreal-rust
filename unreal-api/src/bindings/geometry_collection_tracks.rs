#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCollectionParams {
    pub geometry_collection_cache: crate::bindings::core_u_object::FSoftObjectPath,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub play_rate: f32,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCollectionSectionTemplateParameters {
    pub section_start_time: crate::bindings::core_u_object::FFrameNumber,
    pub section_end_time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCollectionSectionTemplate {
    pub params: FMovieSceneGeometryCollectionSectionTemplateParameters,
}
pub struct UMovieSceneGeometryCollectionSection {
    pub params: FMovieSceneGeometryCollectionParams,
}
pub struct UMovieSceneGeometryCollectionTrack {
    pub animation_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
