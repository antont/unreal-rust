#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UFrameHitchSceneDecoration {
    __padding_end: [u8; 2368],
}
impl UFrameHitchSceneDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneTakeSection {
    __padding_end: [u8; 2416],
}
impl UMovieSceneTakeSection {}
#[repr(C, align(8))]
pub struct UMovieSceneTakeSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub hours_name: FString,
    pub minutes_name: FString,
    pub seconds_name: FString,
    pub frames_name: FString,
    pub sub_frames_name: FString,
    pub rate_name: FString,
    pub slate_name: FString,
}
impl UMovieSceneTakeSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneTakeTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneTakeTrack {}
