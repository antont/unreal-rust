#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimPoseEvaluationOptions {
    pub evaluation_type: EAnimDataEvalType,
    pub b_should_retarget: bool,
    pub b_extract_root_motion: bool,
    pub b_incorporate_root_motion_into_pose: bool,
    pub optional_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_retrieve_additive_as_full_pose: bool,
    pub b_evaluate_curves: bool,
    __padding_end: [u8; 6],
}
impl FAnimPoseEvaluationOptions {}
#[repr(C, align(8))]
pub struct FAnimPose {
    __padding_end: [u8; 192],
}
impl FAnimPose {}
#[repr(C, align(8))]
pub struct UAnimationAttributeBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationAttributeBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UAnimationBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UAnimPoseExtensions {
    __padding_end: [u8; 48],
}
impl UAnimPoseExtensions {}
#[repr(C, align(8))]
pub struct UK2Node_BaseAttributeActionNode {
    __padding_end: [u8; 368],
}
impl UK2Node_BaseAttributeActionNode {}
#[repr(C, align(8))]
pub struct UK2Node_SetAttributeKeyGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_SetAttributeKeyGeneric {}
#[repr(C, align(8))]
pub struct UK2Node_SetAttributeKeysGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_SetAttributeKeysGeneric {}
#[repr(C, align(8))]
pub struct UK2Node_GetAttributeKeyGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_GetAttributeKeyGeneric {}
#[repr(C, align(8))]
pub struct UK2Node_GetAttributeKeysGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_GetAttributeKeysGeneric {}
#[repr(C, align(8))]
pub struct FReplaceAnimNotifies_OnNotifyReplaced {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FReplaceAnimNotifyStates_OnNotifyStateReplaced {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EAnimDataEvalType(pub u8);
impl EAnimDataEvalType {
    pub const SOURCE: EAnimDataEvalType = EAnimDataEvalType(0);
    pub const RAW: EAnimDataEvalType = EAnimDataEvalType(1);
    pub const COMPRESSED: EAnimDataEvalType = EAnimDataEvalType(2);
}
#[repr(transparent)]
pub struct EAnimPoseSpaces(pub u8);
impl EAnimPoseSpaces {
    pub const LOCAL: EAnimPoseSpaces = EAnimPoseSpaces(0);
    pub const WORLD: EAnimPoseSpaces = EAnimPoseSpaces(1);
}
