#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FConstraintsInWorld {
    pub world: TWeakObjectPtr<UWorld>,
    pub constraints: TArray<TWeakObjectPtr<UTickableConstraint>>,
}
#[repr(C, align(16))]
pub struct FMovieSceneConstraintChannel {}
#[repr(C, align(16))]
pub struct FConstraintAndActiveChannel {
    pub active_channel: FMovieSceneConstraintChannel,
    pub constraint_copy_to_spawn: UPtr<UTickableConstraint>,
}
#[repr(C, align(8))]
pub struct FConstraintTickFunction {}
pub struct AConstraintsActor {
    pub constraints_manager: UPtr<UConstraintsManager>,
}
pub struct UTickableConstraint {
    pub active: bool,
    pub b_valid: bool,
}
pub struct UConstraintsManager {
    pub on_constraint_added_bp: FConstraintsManager_OnConstraintAdded_BP,
    pub on_constraint_removed_bp: FConstraintsManager_OnConstraintRemoved_BP,
    pub constraints: TArray<UPtr<UTickableConstraint>>,
}
pub struct UConstraintsScriptingLibrary {}
pub struct UConstraintSubsystem {
    pub on_constraint_added_to_system_bp: FConstraintSubsystem_OnConstraintAddedToSystem_BP,
    pub on_constraint_removed_from_system_bp: FConstraintSubsystem_OnConstraintRemovedFromSystem_BP,
    pub constraints_in_world: TArray<FConstraintsInWorld>,
    pub constraints_config: TMap<TSubclassOf<UObject>, UPtr<UTickableConstraint>>,
}
pub struct UTransformableHandle {
    pub constraint_binding_id: FMovieSceneObjectBindingID,
}
pub struct UTransformableComponentHandle {
    pub component: TWeakObjectPtr<USceneComponent>,
    pub socket_name: FName,
}
pub struct UTickableTransformConstraint {
    pub parent_trs_handle: UPtr<UTransformableHandle>,
    pub child_trs_handle: UPtr<UTransformableHandle>,
    pub b_maintain_offset: bool,
    pub weight: f32,
    pub b_dynamic_offset: bool,
    pub ty: ETransformConstraintType,
    pub b_use_current_offset: bool,
}
pub struct UTickableTranslationConstraint {
    pub offset_translation: FVector,
    pub axis_filter: FFilterOptionPerAxis,
}
pub struct UTickableRotationConstraint {
    pub offset_rotation: FQuat,
    pub axis_filter: FFilterOptionPerAxis,
}
pub struct UTickableScaleConstraint {
    pub offset_scale: FVector,
    pub axis_filter: FFilterOptionPerAxis,
}
pub struct UTickableParentConstraint {
    pub offset_transform: FTransform,
    pub b_scaling: bool,
    pub transform_filter: FTransformFilter,
}
pub struct UTickableLookAtConstraint {
    pub axis: FVector,
}
