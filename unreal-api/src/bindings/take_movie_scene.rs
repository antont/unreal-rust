#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTakeMovieSceneHitchTimecodeCurves {
    pub hours_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub minutes_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub seconds_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub frames_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
}
pub struct UFrameHitchSceneDecoration {
    pub target_timecode: FTakeMovieSceneHitchTimecodeCurves,
    pub actual_timecode: FTakeMovieSceneHitchTimecodeCurves,
    pub timecode_provider_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub record_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
pub struct UMovieSceneTakeSection {
    pub hours_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub minutes_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub seconds_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub frames_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
    pub sub_frames_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub rate_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub slate: crate::bindings::movie_scene_tracks::FMovieSceneStringChannel,
}
pub struct UMovieSceneTakeSettings {
    pub hours_name: FString,
    pub minutes_name: FString,
    pub seconds_name: FString,
    pub frames_name: FString,
    pub sub_frames_name: FString,
    pub rate_name: FString,
    pub slate_name: FString,
}
pub struct UMovieSceneTakeTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
