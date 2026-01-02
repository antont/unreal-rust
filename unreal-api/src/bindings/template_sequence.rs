#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FTemplateSequenceBindingOverrideData {
    __padding_end: [u8; 12],
}
impl FTemplateSequenceBindingOverrideData {}
#[repr(C, align(8))]
pub struct UTemplateSequence {
    __padding_end: [u8; 264],
}
impl UTemplateSequence {}
#[repr(C, align(8))]
pub struct UCameraAnimationSequence {
    __padding_end: [u8; 264],
}
impl UCameraAnimationSequence {}
#[repr(C, align(16))]
pub struct UCameraAnimationSequenceCameraStandIn {
    __padding_end: [u8; 2320],
}
impl UCameraAnimationSequenceCameraStandIn {}
#[repr(C, align(8))]
pub struct UCameraAnimationSequencePlayer {
    __padding_end: [u8; 824],
}
impl UCameraAnimationSequencePlayer {}
#[repr(C, align(8))]
pub struct UCameraAnimationSpawnableSystem {
    __padding_end: [u8; 80],
}
impl UCameraAnimationSpawnableSystem {}
#[repr(C, align(8))]
pub struct UCameraAnimationBoundObjectInstantiator {
    __padding_end: [u8; 80],
}
impl UCameraAnimationBoundObjectInstantiator {}
#[repr(C, align(8))]
pub struct UCameraAnimationEntitySystemLinker {
    __padding_end: [u8; 1952],
}
impl UCameraAnimationEntitySystemLinker {}
#[repr(C, align(8))]
pub struct UCameraAnimationSequenceSubsystem {
    __padding_end: [u8; 88],
}
impl UCameraAnimationSequenceSubsystem {}
#[repr(C, align(8))]
pub struct UTemplateSequenceSection {
    __padding_end: [u8; 2440],
}
impl UTemplateSequenceSection {}
#[repr(C, align(8))]
pub struct USequenceCameraShakePattern {
    __padding_end: [u8; 128],
}
impl USequenceCameraShakePattern {}
#[repr(C, align(8))]
pub struct UTemplateSequenceSystem {
    __padding_end: [u8; 224],
}
impl UTemplateSequenceSystem {}
#[repr(C, align(8))]
pub struct UTemplateSequencePropertyScalingInstantiatorSystem {
    __padding_end: [u8; 168],
}
impl UTemplateSequencePropertyScalingInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UTemplateSequencePropertyScalingEvaluatorSystem {
    __padding_end: [u8; 160],
}
impl UTemplateSequencePropertyScalingEvaluatorSystem {}
#[repr(C, align(8))]
pub struct ATemplateSequenceActor {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<UTemplateSequencePlayer>,
    pub template_sequence: crate::bindings::core_u_object::FSoftObjectPath,
    pub binding_override: FTemplateSequenceBindingOverrideData,
    __padding_end: [u8; 4],
}
impl ATemplateSequenceActor {}
#[repr(C, align(8))]
pub struct UTemplateSequencePlayer {
    __padding_end: [u8; 1224],
}
impl UTemplateSequencePlayer {}
#[repr(C, align(8))]
pub struct USequenceCameraShakeTestUtil {
    __padding_end: [u8; 48],
}
impl USequenceCameraShakeTestUtil {}
#[repr(C, align(8))]
pub struct UTemplateSequenceTrack {
    __padding_end: [u8; 416],
}
impl UTemplateSequenceTrack {}
#[repr(transparent)]
pub struct ETemplateSectionPropertyScaleType(pub i32);
impl ETemplateSectionPropertyScaleType {
    pub const FLOAT_PROPERTY: ETemplateSectionPropertyScaleType = ETemplateSectionPropertyScaleType(
        0,
    );
    pub const TRANSFORM_PROPERTY_LOCATION_ONLY: ETemplateSectionPropertyScaleType = ETemplateSectionPropertyScaleType(
        1,
    );
    pub const TRANSFORM_PROPERTY_ROTATION_ONLY: ETemplateSectionPropertyScaleType = ETemplateSectionPropertyScaleType(
        2,
    );
}
