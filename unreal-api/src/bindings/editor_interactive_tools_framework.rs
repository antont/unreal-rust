#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FRotationContext {
    pub b_use_explicit_rotator: bool,
    pub rotation_order: EEulerRotationOrder,
    pub rotation: FRotator,
    pub offset: FTransform,
}
#[repr(C, align(4))]
pub struct FGizmosParameters {
    pub rotate_mode: EAxisRotateMode,
    pub b_ctrl_middle_does_y: bool,
    pub b_enable_explicit: bool,
    pub axis_size_multiplier: f32,
}
#[repr(C, align(8))]
pub struct FActiveEditorGizmo {
    pub gizmo: UPtr<UInteractiveGizmo>,
}
#[repr(C, align(8))]
pub struct FInteractiveToolStack {}
pub struct UEditorInteractiveGizmoConditionalBuilder {}
pub struct IEditorInteractiveGizmoConditionalBuilder {}
pub struct UEditorSceneSnappingManager {}
pub struct UGizmoEdModeInterface {}
pub struct IGizmoEdModeInterface {}
pub struct UTransformGizmoSource {}
pub struct ITransformGizmoSource {}
pub struct UViewportInteraction {
    pub viewport_interactions_behavior_source: TWeakObjectPtr<
        UViewportInteractionsBehaviorSource,
    >,
    pub input_behaviors: TArray<UPtr<UInputBehavior>>,
}
pub struct UViewportCameraRotateInteraction {}
pub struct UViewportCameraSpeedMouseWheelInteraction {}
pub struct UViewportCameraTranslateInteraction {}
pub struct UMultiButtonClickDragBehavior {}
pub struct UViewportClickDragBehavior {}
pub struct UViewportCommandsInteraction {}
pub struct UViewportDragInteraction {}
pub struct UViewportFOVInteraction {}
pub struct UViewportInteractionsBehaviorSource {
    pub behavior_set: UPtr<UInputBehaviorSet>,
    pub viewport_interactions: TMap<FName, UPtr<UViewportInteraction>>,
    pub editor_interactive_tools_context_weak: TWeakObjectPtr<
        UEditorInteractiveToolsContext,
    >,
}
pub struct UViewportMoveYawInteraction {}
pub struct UViewportOrbitInteraction {}
pub struct UViewportOrthoPanInteraction {}
pub struct UViewportPanInteraction {}
pub struct UViewportViewAngleInteraction {}
pub struct UViewportZoomInteraction {}
pub struct UDragToolsBehaviorSource {
    pub behavior_set: UPtr<UInputBehaviorSet>,
}
pub struct UEditorGizmoStateTarget {
    pub transaction_manager: TScriptInterface<IToolContextTransactionProvider>,
}
pub struct UTransformGizmo {
    pub active_target: UPtr<UTransformProxy>,
    pub hit_target: UPtr<UGizmoElementHitMultiTarget>,
    pub multi_indirect_click_drag_behavior: UPtr<UMultiButtonClickDragBehavior>,
    pub transform_gizmo_source: TScriptInterface<ITransformGizmoSource>,
    pub gizmo_element_root: UPtr<UGizmoElementGroup>,
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
    pub b_visible: bool,
    pub b_in_interaction: bool,
    pub b_snap_to_world_grid: bool,
    pub b_grid_size_is_explicit: bool,
    pub explicit_grid_size: FVector,
    pub b_rotation_grid_size_is_explicit: bool,
    pub explicit_rotation_grid_size: FRotator,
    pub b_snap_to_world_rot_grid: bool,
    pub translate_x_axis_element: UPtr<UGizmoElementArrow>,
    pub translate_y_axis_element: UPtr<UGizmoElementArrow>,
    pub translate_z_axis_element: UPtr<UGizmoElementArrow>,
    pub translate_screen_space_element: UPtr<UGizmoElementRectangle>,
    pub translate_planar_xy_element: UPtr<UGizmoElementBox>,
    pub translate_planar_yz_element: UPtr<UGizmoElementBox>,
    pub translate_planar_xz_element: UPtr<UGizmoElementBox>,
    pub rotate_x_axis_element: UPtr<UGizmoElementTorus>,
    pub rotate_y_axis_element: UPtr<UGizmoElementTorus>,
    pub rotate_z_axis_element: UPtr<UGizmoElementTorus>,
    pub rotate_x_gimbal_element: UPtr<UGizmoElementTorus>,
    pub rotate_y_gimbal_element: UPtr<UGizmoElementTorus>,
    pub rotate_z_gimbal_element: UPtr<UGizmoElementTorus>,
    pub rotate_gimbal_element: UPtr<UGizmoElementGimbal>,
    pub rotate_arcball_element: UPtr<UGizmoElementCircle>,
    pub rotate_screen_space_element: UPtr<UGizmoElementCircle>,
    pub scale_x_axis_element: UPtr<UGizmoElementArrow>,
    pub scale_y_axis_element: UPtr<UGizmoElementArrow>,
    pub scale_z_axis_element: UPtr<UGizmoElementArrow>,
    pub scale_planar_xy_element: UPtr<UGizmoElementBox>,
    pub scale_planar_yz_element: UPtr<UGizmoElementBox>,
    pub scale_planar_xz_element: UPtr<UGizmoElementBox>,
    pub scale_uniform_element: UPtr<UGizmoElementBox>,
    pub camera_axis_source: UPtr<UGizmoConstantFrameAxisSource>,
    pub state_target: TScriptInterface<IGizmoStateTarget>,
    pub transparent_vertex_color_material: UPtr<UMaterialInterface>,
    pub grid_material: UPtr<UMaterialInterface>,
    pub axis_material_x: UPtr<UMaterialInstanceDynamic>,
    pub axis_material_y: UPtr<UMaterialInstanceDynamic>,
    pub axis_material_z: UPtr<UMaterialInstanceDynamic>,
    pub current_axis_material: UPtr<UMaterialInstanceDynamic>,
    pub grey_material: UPtr<UMaterialInstanceDynamic>,
    pub white_material: UPtr<UMaterialInstanceDynamic>,
    pub opaque_plane_material_xy: UPtr<UMaterialInstanceDynamic>,
    pub scale_multiplier: f64,
    pub current_transform: FTransform,
    pub current_mode: EGizmoTransformMode,
    pub current_axis_to_draw: EAxisList,
    pub last_hit_part: ETransformGizmoPartIdentifier,
    pub interaction_axis_list: EAxisList,
    pub interaction_axis_origin: FVector,
    pub interaction_axis_direction: FVector,
    pub interaction_axis_start_param: f64,
    pub interaction_axis_curr_param: f64,
    pub interaction_planar_origin: FVector,
    pub interaction_planar_normal: FVector,
    pub normal_to_remove: FVector,
    pub interaction_planar_axis_x: FVector,
    pub interaction_planar_axis_y: FVector,
    pub interaction_planar_start_point: FVector,
    pub interaction_planar_curr_point: FVector,
    pub interaction_start_angle: f64,
    pub interaction_curr_angle: f64,
    pub interaction_screen_axis_direction: FVector2D,
    pub normal_projection_to_remove: FVector2D,
    pub interaction_screen_start_pos: FVector2D,
    pub interaction_screen_end_pos: FVector2D,
    pub interaction_screen_curr_pos: FVector2D,
    pub interaction_arc_ball_start_point: FVector,
    pub interaction_arc_ball_curr_point: FVector,
    pub start_rotation: FQuat,
    pub b_indirect_manipulation: bool,
    pub b_defer_drag: bool,
    pub b_ctrl_middle_does_y: bool,
    pub default_rotate_mode: EAxisRotateMode,
}
pub struct UEditorTransformGizmo {}
pub struct UEditorTransformGizmoBuilder {}
pub struct UEditorTransformGizmoSource {}
pub struct UEditorTransformGizmoContextObject {
    pub rotation_context: FRotationContext,
}
pub struct UEditorTransformProxy {
    pub b_use_legacy_widget_scale: bool,
    pub viewport_interaction: UPtr<UTypedElementViewportInteraction>,
}
pub struct UGizmoElementGimbal {
    pub rotation_context: FRotationContext,
}
pub struct UEditorInteractiveGizmoManager {
    pub registry: UPtr<UEditorInteractiveGizmoRegistry>,
    pub active_editor_gizmos: TArray<FActiveEditorGizmo>,
    pub default_gizmo_parameters: TOptional<FGizmosParameters>,
    pub cached_gizmo_map: TMap<UPtr<UInteractiveGizmoBuilder>, UPtr<UInteractiveGizmo>>,
}
pub struct UEditorGizmoRegistryCategoryEntry {
    pub gizmo_types: TArray<UPtr<UInteractiveGizmoBuilder>>,
    pub category_name: FString,
}
pub struct UEditorGizmoRegistryCategoryEntry_ConditionalSelection {}
pub struct UEditorGizmoRegistryCategoryEntry_Primary {}
pub struct UEditorGizmoRegistryCategoryEntry_Accessory {}
pub struct UEditorInteractiveGizmoRegistry {
    pub gizmo_category_map: TMap<
        EEditorGizmoCategory,
        UPtr<UEditorGizmoRegistryCategoryEntry>,
    >,
}
pub struct UEditorInteractiveGizmoSelectionBuilder {}
pub struct IEditorInteractiveGizmoSelectionBuilder {}
pub struct UEditorInteractiveGizmoSubsystem {
    pub transform_gizmo_builder: UPtr<UInteractiveGizmoBuilder>,
    pub registry: UPtr<UEditorInteractiveGizmoRegistry>,
}
pub struct UToolStackContext {
    pub ed_mode: TWeakObjectPtr<UEdMode>,
    pub tool_stacks: TMap<FString, FInteractiveToolStack>,
}
pub struct UWidgetToolsContext {}
pub struct UTransformGizmoEditorSettings {
    pub transform_gizmo_size: f32,
    pub b_enable_arcball_rotate: bool,
    pub b_enable_screen_rotate: bool,
    pub b_enable_axis_drawing: bool,
    pub b_enable_combined_translate_rotate: bool,
    pub b_use_experimental_gizmo: bool,
    pub gizmos_parameters: FGizmosParameters,
}
