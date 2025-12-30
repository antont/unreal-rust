#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FTakeMovieSceneHitchTimecodeCurves {
    pub hours_curve: FMovieSceneIntegerChannel,
    pub minutes_curve: FMovieSceneIntegerChannel,
    pub seconds_curve: FMovieSceneIntegerChannel,
    pub frames_curve: FMovieSceneIntegerChannel,
}
pub struct UFrameHitchSceneDecoration {
    pub target_timecode: FTakeMovieSceneHitchTimecodeCurves,
    pub actual_timecode: FTakeMovieSceneHitchTimecodeCurves,
    pub timecode_provider_frame_rate: FFrameRate,
    pub record_frame_rate: FFrameRate,
}
pub struct UMovieSceneTakeSection {
    pub hours_curve: FMovieSceneIntegerChannel,
    pub minutes_curve: FMovieSceneIntegerChannel,
    pub seconds_curve: FMovieSceneIntegerChannel,
    pub frames_curve: FMovieSceneIntegerChannel,
    pub sub_frames_curve: FMovieSceneFloatChannel,
    pub rate_curve: FMovieSceneFloatChannel,
    pub slate: FMovieSceneStringChannel,
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
    pub sections: TArray<UPtr<UMovieSceneSection>>,
}
