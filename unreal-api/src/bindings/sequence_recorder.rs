#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FTimecodeBoneMethod {
    pub bone_mode: ETimecodeBoneMode,
    pub bone_name: FName,
}
impl FTimecodeBoneMethod {}
#[repr(C, align(8))]
pub struct UMovieScene3DTransformSectionRecorderSettings {
    __padding_end: [u8; 56],
}
impl UMovieScene3DTransformSectionRecorderSettings {}
#[repr(C, align(8))]
pub struct USequenceRecordingBase {
    __padding_end: [u8; 48],
}
impl USequenceRecordingBase {}
#[repr(C, align(8))]
pub struct UActorRecording {
    __padding_end: [u8; 376],
}
impl UActorRecording {}
#[repr(C, align(8))]
pub struct UAnimationRecordingParameters {
    __padding_end: [u8; 72],
}
impl UAnimationRecordingParameters {}
#[repr(C, align(8))]
pub struct USequenceRecorderActorGroup {
    __padding_end: [u8; 136],
}
impl USequenceRecorderActorGroup {}
#[repr(C, align(8))]
pub struct ASequenceRecorderGroup {
    __padding_end: [u8; 1152],
}
impl ASequenceRecorderGroup {}
#[repr(C, align(8))]
pub struct USequenceRecorderBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USequenceRecorderBlueprintLibrary {}
#[repr(C, align(8))]
pub struct USequenceRecorderSettings {
    __padding_end: [u8; 304],
}
impl USequenceRecorderSettings {}
#[repr(transparent)]
pub struct ETimecodeBoneMode(pub u8);
impl ETimecodeBoneMode {
    pub const ALL: ETimecodeBoneMode = ETimecodeBoneMode(0);
    pub const ROOT: ETimecodeBoneMode = ETimecodeBoneMode(1);
    pub const USER_DEFINED: ETimecodeBoneMode = ETimecodeBoneMode(2);
    pub const MAX: ETimecodeBoneMode = ETimecodeBoneMode(3);
}
#[repr(transparent)]
pub struct EAudioRecordingMode(pub u8);
impl EAudioRecordingMode {
    pub const NONE: EAudioRecordingMode = EAudioRecordingMode(0);
    pub const AUDIO_TRACK: EAudioRecordingMode = EAudioRecordingMode(1);
}
