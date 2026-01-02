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
    __padding_end: [u8; 8],
}
impl FSequencerScriptingRange {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingKey {
    __padding_end: [u8; 72],
}
impl UMovieSceneScriptingKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingActorReferenceKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingActorReferenceKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingChannel {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub channel_name: FName,
    __padding_end: [u8; 4],
}
impl UMovieSceneScriptingChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingActorReferenceChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingActorReferenceChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingBoolKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingBoolKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingBoolChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingBoolChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingByteKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingByteKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingByteChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingByteChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingDoubleKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingDoubleKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingDoubleChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingDoubleChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingEventKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingEventKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingEventChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingEventChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingFloatKey {
    __padding_end: [u8; 72],
}
impl UMovieSceneScriptingFloatKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingActualFloatKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingActualFloatKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingDoubleAsFloatKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingDoubleAsFloatKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingFloatChannel {
    __padding_end: [u8; 144],
}
impl UMovieSceneScriptingFloatChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingIntegerKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingIntegerKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingIntegerChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingIntegerChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingObjectPathKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingObjectPathKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingObjectPathChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingObjectPathChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingParticleKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingParticleKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingParticleChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingParticleChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingStringKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingStringKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingStringChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingStringChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingTextKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingTextKey {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingTextChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingTextChannel {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneBindingExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneEventTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneEventTrackExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneFolderExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneFolderExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneMaterialTrackExtensions {}
#[repr(C, align(8))]
pub struct UMovieScenePrimitiveMaterialTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieScenePrimitiveMaterialTrackExtensions {}
#[repr(C, align(8))]
pub struct UMovieScenePropertyTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieScenePropertyTrackExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneSectionExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneSectionExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneSequenceExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneSequenceExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneTimeWarpExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatVectorTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneFloatVectorTrackExtensions {}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleVectorTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneDoubleVectorTrackExtensions {}
#[repr(C, align(8))]
pub struct USequencerScriptingRangeExtensions {
    __padding_end: [u8; 48],
}
impl USequencerScriptingRangeExtensions {}
