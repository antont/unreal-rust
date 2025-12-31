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
    pub b_is_input_captured: bool,
    pub b_is_axis: bool,
}
#[repr(C, align(4))]
pub struct FTransformGizmoHandlePlacement {}
#[repr(C, align(8))]
pub struct FTransformGizmoMeasurement {
    pub measurement_text: UPtr<crate::bindings::engine::UTextRenderComponent>,
}
#[repr(C, align(8))]
pub struct FGizmoHandle {}
#[repr(C, align(16))]
pub struct FDraggingTransformableData {}
pub struct UViewportDragOperation {}
pub struct UViewportInteractor {
    pub key_to_action_map: TMap<
        crate::bindings::input_core::FKey,
        FViewportActionKeyInput,
    >,
    pub world_interaction: UPtr<UViewportWorldInteraction>,
    pub other_interactor: UPtr<UViewportInteractor>,
}
pub struct UVISettings {
    pub flags_48: u8,
}
pub struct UViewportInteractionAssetContainer {
    pub gizmo_handle_selected_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub gizmo_handle_drop_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub selection_change_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub selection_drop_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub selection_start_drag_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub grid_snap_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub actor_snap_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub undo_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub redo_sound: UPtr<crate::bindings::engine::USoundBase>,
    pub grid_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub translation_handle_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub uniform_scale_handle_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub scale_handle_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub plane_translation_handle_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub rotation_handle_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub rotation_handle_selected_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub start_rotation_indicator_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub current_rotation_indicator_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub free_rotation_handle_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub grid_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub transform_gizmo_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub translucent_transform_gizmo_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
}
pub struct UViewportTransformer {
    pub viewport_world_interaction: UPtr<UViewportWorldInteraction>,
}
pub struct UActorTransformer {}
pub struct ABaseTransformGizmo {
    pub scene_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub all_handle_groups: TArray<UPtr<UGizmoHandleGroup>>,
    pub world_interaction: UPtr<UViewportWorldInteraction>,
}
pub struct UGizmoHandleGroup {
    pub gizmo_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub translucent_gizmo_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub handles: TArray<FGizmoHandle>,
    pub owning_transform_gizmo_actor: UPtr<ABaseTransformGizmo>,
    pub drag_operation_component: UPtr<UViewportDragOperationComponent>,
}
pub struct UAxisGizmoHandleGroup {}
pub struct UGizmoHandleMeshComponent {}
pub struct APivotTransformGizmo {
    pub uniform_scale_gizmo_handle_group: UPtr<UUniformScaleGizmoHandleGroup>,
    pub translation_gizmo_handle_group: UPtr<UPivotTranslationGizmoHandleGroup>,
    pub scale_gizmo_handle_group: UPtr<UPivotScaleGizmoHandleGroup>,
    pub plane_translation_gizmo_handle_group: UPtr<
        UPivotPlaneTranslationGizmoHandleGroup,
    >,
    pub rotation_gizmo_handle_group: UPtr<UPivotRotationGizmoHandleGroup>,
    pub stretch_gizmo_handle_group: UPtr<UStretchGizmoHandleGroup>,
    pub last_dragging_handle: UPtr<crate::bindings::engine::UActorComponent>,
}
pub struct UPivotTranslationGizmoHandleGroup {}
pub struct UPivotScaleGizmoHandleGroup {}
pub struct UPivotPlaneTranslationGizmoHandleGroup {}
pub struct UPivotRotationGizmoHandleGroup {
    pub root_full_rotation_handle_component: UPtr<
        crate::bindings::engine::USceneComponent,
    >,
    pub full_rotation_handle_mesh_component: UPtr<UGizmoHandleMeshComponent>,
    pub start_rotation_indicator_mesh_component: UPtr<UGizmoHandleMeshComponent>,
    pub root_start_rotation_idicator_component: UPtr<
        crate::bindings::engine::USceneComponent,
    >,
    pub delta_rotation_indicator_mesh_component: UPtr<UGizmoHandleMeshComponent>,
    pub root_delta_rotation_indicator_component: UPtr<
        crate::bindings::engine::USceneComponent,
    >,
}
pub struct UStretchGizmoHandleGroup {}
pub struct UStretchGizmoHandleDragOperation {}
pub struct UUniformScaleGizmoHandleGroup {}
pub struct UMouseCursorInteractor {}
pub struct UViewportDragOperationComponent {
    pub drag_operation: UPtr<UViewportDragOperation>,
    pub drag_operation_subclass: TSubclassOf<UViewportDragOperation>,
}
pub struct UViewportInteractableInterface {}
pub struct IViewportInteractableInterface {}
pub struct UTranslationDragOperation {}
pub struct UPlaneTranslationDragOperation {}
pub struct URotateOnAngleDragOperation {}
pub struct UScaleDragOperation {}
pub struct UUniformScaleDragOperation {}
pub struct UViewportWorldInteraction {
    pub interactors: TArray<UPtr<UViewportInteractor>>,
    pub viewport_transformer: UPtr<UViewportTransformer>,
    pub transform_gizmo_actor: UPtr<ABaseTransformGizmo>,
    pub snap_grid_actor: UPtr<crate::bindings::engine::AActor>,
    pub snap_grid_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub snap_grid_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub default_mouse_cursor_interactor: UPtr<UMouseCursorInteractor>,
    pub actors_to_exclude_from_hit_test: TArray<
        TWeakObjectPtr<crate::bindings::engine::AActor>,
    >,
    pub asset_container: UPtr<UViewportInteractionAssetContainer>,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHitResultGizmoFilterMode(pub u8);
impl EHitResultGizmoFilterMode {
    pub const ALL: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(0);
    pub const NO_GIZMOS: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(1);
    pub const GIZMOS_ONLY: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoHandleTypes(pub u8);
impl EGizmoHandleTypes {
    pub const ALL: EGizmoHandleTypes = EGizmoHandleTypes(0);
    pub const TRANSLATE: EGizmoHandleTypes = EGizmoHandleTypes(1);
    pub const ROTATE: EGizmoHandleTypes = EGizmoHandleTypes(2);
    pub const SCALE: EGizmoHandleTypes = EGizmoHandleTypes(3);
}
