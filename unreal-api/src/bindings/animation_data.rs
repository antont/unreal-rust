#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimationCurveMetaData {
    pub flags: i32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub comment: FString,
}
pub struct UAnimSequencerController {
    pub model: TWeakObjectPtr<UAnimationSequencerDataModel>,
    pub model_interface: TScriptInterface<crate::bindings::engine::IAnimationDataModel>,
}
pub struct UAnimationSequencerDataModel {
    pub modified_event_dynamic: FAnimationSequencerDataModel_ModifiedEventDynamic,
    pub legacy_curve_data: crate::bindings::engine::FAnimationCurveData,
    pub animated_bone_attributes: TArray<
        crate::bindings::engine::FAnimatedBoneAttribute,
    >,
    pub movie_scene: UPtr<crate::bindings::movie_scene::UMovieScene>,
    pub curve_identifier_to_meta_data: TMap<
        crate::bindings::engine::FAnimationCurveIdentifier,
        FAnimationCurveMetaData,
    >,
    pub b_populated: bool,
    pub cached_raw_data_guid: crate::bindings::core_u_object::FGuid,
    pub b_rig_hierarchy_initialized: bool,
}
pub struct FAnimationSequencerDataModel_ModifiedEventDynamic;
