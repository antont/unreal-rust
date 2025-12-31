#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FSequencerScriptingRange {
    pub flags_0: u8,
    pub inclusive_start: i32,
    pub exclusive_end: i32,
    pub internal_rate: crate::bindings::core_u_object::FFrameRate,
}
pub struct UMovieSceneScriptingKey {}
pub struct UMovieSceneScriptingActorReferenceKey {}
pub struct UMovieSceneScriptingChannel {
    pub channel_name: FName,
}
pub struct UMovieSceneScriptingActorReferenceChannel {}
pub struct UMovieSceneScriptingBoolKey {}
pub struct UMovieSceneScriptingBoolChannel {}
pub struct UMovieSceneScriptingByteKey {}
pub struct UMovieSceneScriptingByteChannel {}
pub struct UMovieSceneScriptingDoubleKey {}
pub struct UMovieSceneScriptingDoubleChannel {}
pub struct UMovieSceneScriptingEventKey {}
pub struct UMovieSceneScriptingEventChannel {}
pub struct UMovieSceneScriptingFloatKey {}
pub struct UMovieSceneScriptingActualFloatKey {}
pub struct UMovieSceneScriptingDoubleAsFloatKey {}
pub struct UMovieSceneScriptingFloatChannel {}
pub struct UMovieSceneScriptingIntegerKey {}
pub struct UMovieSceneScriptingIntegerChannel {}
pub struct UMovieSceneScriptingObjectPathKey {}
pub struct UMovieSceneScriptingObjectPathChannel {}
pub struct UMovieSceneScriptingParticleKey {}
pub struct UMovieSceneScriptingParticleChannel {}
pub struct UMovieSceneScriptingStringKey {}
pub struct UMovieSceneScriptingStringChannel {}
pub struct UMovieSceneScriptingTextKey {}
pub struct UMovieSceneScriptingTextChannel {}
pub struct UMovieSceneBindingExtensions {}
pub struct UMovieSceneEventTrackExtensions {}
pub struct UMovieSceneFolderExtensions {}
pub struct UMovieSceneMaterialTrackExtensions {}
pub struct UMovieScenePrimitiveMaterialTrackExtensions {}
pub struct UMovieScenePropertyTrackExtensions {}
pub struct UMovieSceneSectionExtensions {}
pub struct UMovieSceneSequenceExtensions {}
pub struct UMovieSceneTimeWarpExtensions {}
pub struct UMovieSceneTrackExtensions {}
pub struct UMovieSceneFloatVectorTrackExtensions {}
pub struct UMovieSceneDoubleVectorTrackExtensions {}
pub struct USequencerScriptingRangeExtensions {}
