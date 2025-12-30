#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCollectionParams {
    pub geometry_collection_cache: FSoftObjectPath,
    pub start_frame_offset: FFrameNumber,
    pub end_frame_offset: FFrameNumber,
    pub play_rate: f32,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCollectionSectionTemplateParameters {
    pub section_start_time: FFrameNumber,
    pub section_end_time: FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCollectionSectionTemplate {
    pub params: FMovieSceneGeometryCollectionSectionTemplateParameters,
}
pub struct UMovieSceneGeometryCollectionSection {
    pub params: FMovieSceneGeometryCollectionParams,
}
pub struct UMovieSceneGeometryCollectionTrack {
    pub animation_sections: TArray<UPtr<UMovieSceneSection>>,
}
