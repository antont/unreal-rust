#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAnimationCurveMetaData {
    pub flags: i32,
    pub color: FLinearColor,
    pub comment: FString,
}
pub struct UAnimSequencerController {
    pub model: TWeakObjectPtr<UAnimationSequencerDataModel>,
    pub model_interface: TScriptInterface<IAnimationDataModel>,
}
pub struct UAnimationSequencerDataModel {
    pub modified_event_dynamic: FAnimationSequencerDataModel_ModifiedEventDynamic,
    pub legacy_curve_data: FAnimationCurveData,
    pub animated_bone_attributes: TArray<FAnimatedBoneAttribute>,
    pub movie_scene: UPtr<UMovieScene>,
    pub curve_identifier_to_meta_data: TMap<
        FAnimationCurveIdentifier,
        FAnimationCurveMetaData,
    >,
    pub b_populated: bool,
    pub cached_raw_data_guid: FGuid,
    pub b_rig_hierarchy_initialized: bool,
}
