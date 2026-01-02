#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMovieSceneGeometryCollectionSection {
    __padding_end: [u8; 432],
}
impl UMovieSceneGeometryCollectionSection {}
#[repr(C, align(8))]
pub struct UMovieSceneGeometryCollectionTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneGeometryCollectionTrack {}
