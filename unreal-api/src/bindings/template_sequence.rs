#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTemplateSectionPropertyScale {
    pub object_binding: crate::bindings::core_u_object::FGuid,
    pub property_binding: crate::bindings::movie_scene::FMovieScenePropertyBinding,
    pub property_scale_type: ETemplateSectionPropertyScaleType,
    pub float_channel: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
#[repr(C, align(4))]
pub struct FTemplateSequenceBindingOverrideData {
    pub object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub b_overrides_default: bool,
}
pub struct UTemplateSequence {
    pub movie_scene: UPtr<crate::bindings::movie_scene::UMovieScene>,
    pub bound_actor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub bound_actor_components: TMap<crate::bindings::core_u_object::FGuid, FName>,
}
pub struct UCameraAnimationSequence {}
pub struct UCameraAnimationSequenceCameraStandIn {
    pub field_of_view: f32,
    pub flags_52: u8,
    pub aspect_ratio: f32,
    pub post_process_settings: crate::bindings::engine::FPostProcessSettings,
    pub post_process_blend_weight: f32,
    pub filmback: crate::bindings::cinematic_camera::FCameraFilmbackSettings,
    pub lens_settings: crate::bindings::cinematic_camera::FCameraLensSettings,
    pub focus_settings: crate::bindings::cinematic_camera::FCameraFocusSettings,
    pub current_focal_length: f32,
    pub current_aperture: f32,
    pub current_focus_distance: f32,
}
pub struct UCameraAnimationSequencePlayer {
    pub bound_object_override: UPtr<crate::bindings::core_u_object::UObject>,
    pub sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    pub root_template_instance: crate::bindings::movie_scene::FMovieSceneRootEvaluationTemplateInstance,
}
pub struct UCameraAnimationSpawnableSystem {}
pub struct UCameraAnimationBoundObjectInstantiator {}
pub struct UCameraAnimationEntitySystemLinker {}
pub struct UCameraAnimationSequenceSubsystem {
    pub linker: UPtr<crate::bindings::movie_scene::UMovieSceneEntitySystemLinker>,
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
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<UTemplateSequencePlayer>,
    pub template_sequence: crate::bindings::core_u_object::FSoftObjectPath,
    pub binding_override: FTemplateSequenceBindingOverrideData,
}
pub struct UTemplateSequencePlayer {}
pub struct USequenceCameraShakeTestUtil {}
pub struct UTemplateSequenceTrack {}
#[allow(non_camel_case_types)]
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
