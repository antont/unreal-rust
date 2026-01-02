#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FViewportActionKeyInput {
    pub action_type: FName,
    pub event: crate::bindings::engine::EInputEvent,
    __padding_end: [u8; 3],
}
impl FViewportActionKeyInput {}
#[repr(C, align(8))]
pub struct UViewportDragOperation {
    __padding_end: [u8; 56],
}
impl UViewportDragOperation {}
#[repr(C, align(16))]
pub struct UViewportInteractor {
    __padding_end: [u8; 1952],
}
impl UViewportInteractor {}
#[repr(C, align(8))]
pub struct UVISettings {
    __padding_end: [u8; 56],
}
impl UVISettings {}
#[repr(C, align(8))]
pub struct UViewportInteractionAssetContainer {
    __padding_end: [u8; 232],
}
impl UViewportInteractionAssetContainer {}
#[repr(C, align(8))]
pub struct UViewportTransformer {
    __padding_end: [u8; 56],
}
impl UViewportTransformer {}
#[repr(C, align(8))]
pub struct UActorTransformer {
    __padding_end: [u8; 56],
}
impl UActorTransformer {}
#[repr(C, align(8))]
pub struct ABaseTransformGizmo {
    __padding_end: [u8; 1184],
}
impl ABaseTransformGizmo {}
#[repr(C, align(16))]
pub struct UGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UAxisGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UAxisGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UGizmoHandleMeshComponent {
    __padding_end: [u8; 1888],
}
impl UGizmoHandleMeshComponent {}
#[repr(C, align(8))]
pub struct APivotTransformGizmo {
    __padding_end: [u8; 1248],
}
impl APivotTransformGizmo {}
#[repr(C, align(16))]
pub struct UPivotTranslationGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotTranslationGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UPivotScaleGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotScaleGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UPivotPlaneTranslationGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotPlaneTranslationGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UPivotRotationGizmoHandleGroup {
    __padding_end: [u8; 816],
}
impl UPivotRotationGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UStretchGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UStretchGizmoHandleGroup {}
#[repr(C, align(8))]
pub struct UStretchGizmoHandleDragOperation {
    __padding_end: [u8; 56],
}
impl UStretchGizmoHandleDragOperation {}
#[repr(C, align(16))]
pub struct UUniformScaleGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UUniformScaleGizmoHandleGroup {}
#[repr(C, align(16))]
pub struct UMouseCursorInteractor {
    __padding_end: [u8; 1952],
}
impl UMouseCursorInteractor {}
#[repr(C, align(8))]
pub struct UViewportDragOperationComponent {
    __padding_end: [u8; 256],
}
impl UViewportDragOperationComponent {}
pub struct UViewportInteractableInterface {}
pub struct IViewportInteractableInterface {}
#[repr(C, align(8))]
pub struct UTranslationDragOperation {
    __padding_end: [u8; 56],
}
impl UTranslationDragOperation {}
#[repr(C, align(8))]
pub struct UPlaneTranslationDragOperation {
    __padding_end: [u8; 56],
}
impl UPlaneTranslationDragOperation {}
#[repr(C, align(8))]
pub struct URotateOnAngleDragOperation {
    __padding_end: [u8; 120],
}
impl URotateOnAngleDragOperation {}
#[repr(C, align(8))]
pub struct UScaleDragOperation {
    __padding_end: [u8; 56],
}
impl UScaleDragOperation {}
#[repr(C, align(8))]
pub struct UUniformScaleDragOperation {
    __padding_end: [u8; 56],
}
impl UUniformScaleDragOperation {}
#[repr(C, align(16))]
pub struct UViewportWorldInteraction {
    __padding_end: [u8; 1376],
}
impl UViewportWorldInteraction {}
#[repr(transparent)]
pub struct EViewportInteractionDraggingMode(pub u8);
impl EViewportInteractionDraggingMode {
    pub const NOTHING: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        0,
    );
    pub const TRANSFORMABLES_WITH_GIZMO: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        1,
    );
    pub const TRANSFORMABLES_AT_LASER_IMPACT: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        2,
    );
    pub const ASSISTING_DRAG: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        3,
    );
    pub const TRANSFORMABLES_FREELY: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        4,
    );
    pub const WORLD: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        5,
    );
    pub const INTERACTABLE: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        6,
    );
    pub const MATERIAL: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        7,
    );
}
#[repr(transparent)]
pub struct EHitResultGizmoFilterMode(pub u8);
impl EHitResultGizmoFilterMode {
    pub const ALL: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(0);
    pub const NO_GIZMOS: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(1);
    pub const GIZMOS_ONLY: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(2);
}
#[repr(transparent)]
pub struct EGizmoHandleTypes(pub u8);
impl EGizmoHandleTypes {
    pub const ALL: EGizmoHandleTypes = EGizmoHandleTypes(0);
    pub const TRANSLATE: EGizmoHandleTypes = EGizmoHandleTypes(1);
    pub const ROTATE: EGizmoHandleTypes = EGizmoHandleTypes(2);
    pub const SCALE: EGizmoHandleTypes = EGizmoHandleTypes(3);
}
