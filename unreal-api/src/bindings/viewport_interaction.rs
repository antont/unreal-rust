#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FViewportActionKeyInput {
    pub action_type: FName,
    pub event: EInputEvent,
    pub b_is_input_captured: bool,
    pub b_is_axis: bool,
}
#[repr(C, align(4))]
pub struct FTransformGizmoHandlePlacement {}
#[repr(C, align(8))]
pub struct FTransformGizmoMeasurement {
    pub measurement_text: UPtr<UTextRenderComponent>,
}
#[repr(C, align(8))]
pub struct FGizmoHandle {}
#[repr(C, align(16))]
pub struct FDraggingTransformableData {}
pub struct UViewportDragOperation {}
pub struct UViewportInteractor {
    pub key_to_action_map: TMap<FKey, FViewportActionKeyInput>,
    pub world_interaction: UPtr<UViewportWorldInteraction>,
    pub other_interactor: UPtr<UViewportInteractor>,
}
pub struct UVISettings {
    pub flags_48: u8,
}
pub struct UViewportInteractionAssetContainer {
    pub gizmo_handle_selected_sound: UPtr<USoundBase>,
    pub gizmo_handle_drop_sound: UPtr<USoundBase>,
    pub selection_change_sound: UPtr<USoundBase>,
    pub selection_drop_sound: UPtr<USoundBase>,
    pub selection_start_drag_sound: UPtr<USoundBase>,
    pub grid_snap_sound: UPtr<USoundBase>,
    pub actor_snap_sound: UPtr<USoundBase>,
    pub undo_sound: UPtr<USoundBase>,
    pub redo_sound: UPtr<USoundBase>,
    pub grid_mesh: UPtr<UStaticMesh>,
    pub translation_handle_mesh: UPtr<UStaticMesh>,
    pub uniform_scale_handle_mesh: UPtr<UStaticMesh>,
    pub scale_handle_mesh: UPtr<UStaticMesh>,
    pub plane_translation_handle_mesh: UPtr<UStaticMesh>,
    pub rotation_handle_mesh: UPtr<UStaticMesh>,
    pub rotation_handle_selected_mesh: UPtr<UStaticMesh>,
    pub start_rotation_indicator_mesh: UPtr<UStaticMesh>,
    pub current_rotation_indicator_mesh: UPtr<UStaticMesh>,
    pub free_rotation_handle_mesh: UPtr<UStaticMesh>,
    pub grid_material: UPtr<UMaterialInterface>,
    pub transform_gizmo_material: UPtr<UMaterialInterface>,
    pub translucent_transform_gizmo_material: UPtr<UMaterialInterface>,
}
pub struct UViewportTransformer {
    pub viewport_world_interaction: UPtr<UViewportWorldInteraction>,
}
pub struct UActorTransformer {}
pub struct ABaseTransformGizmo {
    pub scene_component: UPtr<USceneComponent>,
    pub all_handle_groups: TArray<UPtr<UGizmoHandleGroup>>,
    pub world_interaction: UPtr<UViewportWorldInteraction>,
}
pub struct UGizmoHandleGroup {
    pub gizmo_material: UPtr<UMaterialInterface>,
    pub translucent_gizmo_material: UPtr<UMaterialInterface>,
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
    pub last_dragging_handle: UPtr<UActorComponent>,
}
pub struct UPivotTranslationGizmoHandleGroup {}
pub struct UPivotScaleGizmoHandleGroup {}
pub struct UPivotPlaneTranslationGizmoHandleGroup {}
pub struct UPivotRotationGizmoHandleGroup {
    pub root_full_rotation_handle_component: UPtr<USceneComponent>,
    pub full_rotation_handle_mesh_component: UPtr<UGizmoHandleMeshComponent>,
    pub start_rotation_indicator_mesh_component: UPtr<UGizmoHandleMeshComponent>,
    pub root_start_rotation_idicator_component: UPtr<USceneComponent>,
    pub delta_rotation_indicator_mesh_component: UPtr<UGizmoHandleMeshComponent>,
    pub root_delta_rotation_indicator_component: UPtr<USceneComponent>,
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
    pub snap_grid_actor: UPtr<AActor>,
    pub snap_grid_mesh_component: UPtr<UStaticMeshComponent>,
    pub snap_grid_mid: UPtr<UMaterialInstanceDynamic>,
    pub default_mouse_cursor_interactor: UPtr<UMouseCursorInteractor>,
    pub actors_to_exclude_from_hit_test: TArray<TWeakObjectPtr<AActor>>,
    pub asset_container: UPtr<UViewportInteractionAssetContainer>,
}
