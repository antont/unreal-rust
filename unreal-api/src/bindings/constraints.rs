#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct AConstraintsActor {
    __padding_end: [u8; 1144],
}
impl AConstraintsActor {}
#[repr(C, align(8))]
pub struct UTickableConstraint {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub active: bool,
    __padding_end: [u8; 103],
}
impl UTickableConstraint {}
#[repr(C, align(8))]
pub struct UConstraintsManager {
    __padding_end: [u8; 80],
}
impl UConstraintsManager {}
#[repr(C, align(8))]
pub struct UConstraintsScriptingLibrary {
    __padding_end: [u8; 48],
}
impl UConstraintsScriptingLibrary {}
#[repr(C, align(8))]
pub struct UConstraintSubsystem {
    __padding_end: [u8; 168],
}
impl UConstraintSubsystem {}
#[repr(C, align(8))]
pub struct UTransformableHandle {
    #[doc(hidden)]
    __padding_52: [u8; 52],
    pub constraint_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    __padding_end: [u8; 32],
}
impl UTransformableHandle {}
#[repr(C, align(8))]
pub struct UTransformableComponentHandle {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub socket_name: FName,
    __padding_end: [u8; 4],
}
impl UTransformableComponentHandle {}
#[repr(C, align(8))]
pub struct UTickableTransformConstraint {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub parent_trs_handle: UPtr<UTransformableHandle>,
    pub child_trs_handle: UPtr<UTransformableHandle>,
    pub b_maintain_offset: bool,
    #[doc(hidden)]
    __padding_176: [u8; 7],
    pub b_dynamic_offset: bool,
    __padding_end: [u8; 7],
}
impl UTickableTransformConstraint {}
#[repr(C, align(8))]
pub struct UTickableTranslationConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_translation: crate::bindings::core_u_object::FVector,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    __padding_end: [u8; 5],
}
impl UTickableTranslationConstraint {}
#[repr(C, align(16))]
pub struct UTickableRotationConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_rotation: crate::bindings::core_u_object::FQuat,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    __padding_end: [u8; 13],
}
impl UTickableRotationConstraint {}
#[repr(C, align(8))]
pub struct UTickableScaleConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_scale: crate::bindings::core_u_object::FVector,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    __padding_end: [u8; 5],
}
impl UTickableScaleConstraint {}
#[repr(C, align(16))]
pub struct UTickableParentConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub b_scaling: bool,
    pub transform_filter: crate::bindings::animation_core::FTransformFilter,
    __padding_end: [u8; 6],
}
impl UTickableParentConstraint {}
#[repr(C, align(8))]
pub struct UTickableLookAtConstraint {
    #[doc(hidden)]
    __padding_184: [u8; 184],
    pub axis: crate::bindings::core_u_object::FVector,
}
impl UTickableLookAtConstraint {}
#[repr(C, align(1))]
pub struct FConstraintsManager_OnConstraintAdded_BP {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FConstraintsManager_OnConstraintRemoved_BP {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FConstraintSubsystem_OnConstraintAddedToSystem_BP {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FConstraintSubsystem_OnConstraintRemovedFromSystem_BP {
    _opague: [u8; 1],
}
