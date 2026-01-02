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
    __padding_end: [u8; 55],
}
impl FMovieSceneGeometryCacheParams {}
#[repr(C, align(8))]
pub struct FMovieSceneGeometryCacheSectionTemplateParameters {
    __padding_end: [u8; 88],
}
impl FMovieSceneGeometryCacheSectionTemplateParameters {}
#[repr(C, align(8))]
pub struct UMovieSceneGeometryCacheSection {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub params: FMovieSceneGeometryCacheParams,
    __padding_end: [u8; 8],
}
impl UMovieSceneGeometryCacheSection {}
#[repr(C, align(8))]
pub struct UMovieSceneGeometryCacheTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneGeometryCacheTrack {}
