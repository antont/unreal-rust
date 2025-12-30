#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FTemplateSectionPropertyScale {
    pub object_binding: FGuid,
    pub property_binding: FMovieScenePropertyBinding,
    pub property_scale_type: ETemplateSectionPropertyScaleType,
    pub float_channel: FMovieSceneFloatChannel,
}
#[repr(C, align(4))]
pub struct FTemplateSequenceBindingOverrideData {
    pub object: TWeakObjectPtr<UObject>,
    pub b_overrides_default: bool,
}
pub struct UTemplateSequence {
    pub movie_scene: UPtr<UMovieScene>,
    pub bound_actor_class: TSoftObjectPtr<UClass>,
    pub bound_actor_components: TMap<FGuid, FName>,
}
pub struct UCameraAnimationSequence {}
pub struct UCameraAnimationSequenceCameraStandIn {
    pub field_of_view: f32,
    pub flags_52: u8,
    pub aspect_ratio: f32,
    pub post_process_settings: FPostProcessSettings,
    pub post_process_blend_weight: f32,
    pub filmback: FCameraFilmbackSettings,
    pub lens_settings: FCameraLensSettings,
    pub focus_settings: FCameraFocusSettings,
    pub current_focal_length: f32,
    pub current_aperture: f32,
    pub current_focus_distance: f32,
}
pub struct UCameraAnimationSequencePlayer {
    pub bound_object_override: UPtr<UObject>,
    pub sequence: UPtr<UMovieSceneSequence>,
    pub root_template_instance: FMovieSceneRootEvaluationTemplateInstance,
}
pub struct UCameraAnimationSpawnableSystem {}
pub struct UCameraAnimationBoundObjectInstantiator {}
pub struct UCameraAnimationEntitySystemLinker {}
pub struct UCameraAnimationSequenceSubsystem {
    pub linker: UPtr<UMovieSceneEntitySystemLinker>,
}
pub struct UTemplateSequenceSection {
    pub property_scales: TArray<FTemplateSectionPropertyScale>,
}
pub struct USequenceCameraShakePattern {
    pub sequence: UPtr<UCameraAnimationSequence>,
    pub play_rate: f32,
    pub scale: f32,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub random_segment_duration: f32,
    pub b_random_segment: bool,
    pub player: UPtr<UCameraAnimationSequencePlayer>,
    pub camera_stand_in: UPtr<UCameraAnimationSequenceCameraStandIn>,
}
pub struct UTemplateSequenceSystem {}
pub struct UTemplateSequencePropertyScalingInstantiatorSystem {}
pub struct UTemplateSequencePropertyScalingEvaluatorSystem {}
pub struct ATemplateSequenceActor {
    pub playback_settings: FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<UTemplateSequencePlayer>,
    pub template_sequence: FSoftObjectPath,
    pub binding_override: FTemplateSequenceBindingOverrideData,
}
pub struct UTemplateSequencePlayer {}
pub struct USequenceCameraShakeTestUtil {}
pub struct UTemplateSequenceTrack {}
