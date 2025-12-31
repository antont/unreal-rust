#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FRotationContext {
    pub b_use_explicit_rotator: bool,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub offset: crate::bindings::core_u_object::FTransform,
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
    pub gizmo: UPtr<crate::bindings::interactive_tools_framework::UInteractiveGizmo>,
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
    pub input_behaviors: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UInputBehavior>,
    >,
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
    pub behavior_set: UPtr<
        crate::bindings::interactive_tools_framework::UInputBehaviorSet,
    >,
    pub viewport_interactions: TMap<FName, UPtr<UViewportInteraction>>,
    pub editor_interactive_tools_context_weak: TWeakObjectPtr<
        crate::bindings::unreal_ed::UEditorInteractiveToolsContext,
    >,
}
pub struct UViewportMoveYawInteraction {}
pub struct UViewportOrbitInteraction {}
pub struct UViewportOrthoPanInteraction {}
pub struct UViewportPanInteraction {}
pub struct UViewportViewAngleInteraction {}
pub struct UViewportZoomInteraction {}
pub struct UDragToolsBehaviorSource {
    pub behavior_set: UPtr<
        crate::bindings::interactive_tools_framework::UInputBehaviorSet,
    >,
}
pub struct UEditorGizmoStateTarget {
    pub transaction_manager: TScriptInterface<
        crate::bindings::interactive_tools_framework::IToolContextTransactionProvider,
    >,
}
pub struct UTransformGizmo {
    pub active_target: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub hit_target: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementHitMultiTarget,
    >,
    pub multi_indirect_click_drag_behavior: UPtr<UMultiButtonClickDragBehavior>,
    pub transform_gizmo_source: TScriptInterface<ITransformGizmoSource>,
    pub gizmo_element_root: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementGroup,
    >,
    pub gizmo_view_context: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoViewContext,
    >,
    pub b_visible: bool,
    pub b_in_interaction: bool,
    pub b_snap_to_world_grid: bool,
    pub b_grid_size_is_explicit: bool,
    pub explicit_grid_size: crate::bindings::core_u_object::FVector,
    pub b_rotation_grid_size_is_explicit: bool,
    pub explicit_rotation_grid_size: crate::bindings::core_u_object::FRotator,
    pub b_snap_to_world_rot_grid: bool,
    pub translate_x_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementArrow,
    >,
    pub translate_y_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementArrow,
    >,
    pub translate_z_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementArrow,
    >,
    pub translate_screen_space_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementRectangle,
    >,
    pub translate_planar_xy_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub translate_planar_yz_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub translate_planar_xz_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub rotate_x_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementTorus,
    >,
    pub rotate_y_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementTorus,
    >,
    pub rotate_z_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementTorus,
    >,
    pub rotate_x_gimbal_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementTorus,
    >,
    pub rotate_y_gimbal_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementTorus,
    >,
    pub rotate_z_gimbal_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementTorus,
    >,
    pub rotate_gimbal_element: UPtr<UGizmoElementGimbal>,
    pub rotate_arcball_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementCircle,
    >,
    pub rotate_screen_space_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementCircle,
    >,
    pub scale_x_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementArrow,
    >,
    pub scale_y_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementArrow,
    >,
    pub scale_z_axis_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementArrow,
    >,
    pub scale_planar_xy_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub scale_planar_yz_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub scale_planar_xz_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub scale_uniform_element: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoElementBox,
    >,
    pub camera_axis_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoConstantFrameAxisSource,
    >,
    pub state_target: TScriptInterface<
        crate::bindings::interactive_tools_framework::IGizmoStateTarget,
    >,
    pub transparent_vertex_color_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub grid_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub axis_material_x: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub axis_material_y: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub axis_material_z: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub current_axis_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub grey_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub white_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub opaque_plane_material_xy: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub scale_multiplier: f64,
    pub current_transform: crate::bindings::core_u_object::FTransform,
    pub current_mode: EGizmoTransformMode,
    pub current_axis_to_draw: crate::bindings::core_u_object::EAxisList,
    pub last_hit_part: ETransformGizmoPartIdentifier,
    pub interaction_axis_list: crate::bindings::core_u_object::EAxisList,
    pub interaction_axis_origin: crate::bindings::core_u_object::FVector,
    pub interaction_axis_direction: crate::bindings::core_u_object::FVector,
    pub interaction_axis_start_param: f64,
    pub interaction_axis_curr_param: f64,
    pub interaction_planar_origin: crate::bindings::core_u_object::FVector,
    pub interaction_planar_normal: crate::bindings::core_u_object::FVector,
    pub normal_to_remove: crate::bindings::core_u_object::FVector,
    pub interaction_planar_axis_x: crate::bindings::core_u_object::FVector,
    pub interaction_planar_axis_y: crate::bindings::core_u_object::FVector,
    pub interaction_planar_start_point: crate::bindings::core_u_object::FVector,
    pub interaction_planar_curr_point: crate::bindings::core_u_object::FVector,
    pub interaction_start_angle: f64,
    pub interaction_curr_angle: f64,
    pub interaction_screen_axis_direction: crate::bindings::core_u_object::FVector2D,
    pub normal_projection_to_remove: crate::bindings::core_u_object::FVector2D,
    pub interaction_screen_start_pos: crate::bindings::core_u_object::FVector2D,
    pub interaction_screen_end_pos: crate::bindings::core_u_object::FVector2D,
    pub interaction_screen_curr_pos: crate::bindings::core_u_object::FVector2D,
    pub interaction_arc_ball_start_point: crate::bindings::core_u_object::FVector,
    pub interaction_arc_ball_curr_point: crate::bindings::core_u_object::FVector,
    pub start_rotation: crate::bindings::core_u_object::FQuat,
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
    pub viewport_interaction: UPtr<
        crate::bindings::editor_framework::UTypedElementViewportInteraction,
    >,
}
pub struct UGizmoElementGimbal {
    pub rotation_context: FRotationContext,
}
pub struct UEditorInteractiveGizmoManager {
    pub registry: UPtr<UEditorInteractiveGizmoRegistry>,
    pub active_editor_gizmos: TArray<FActiveEditorGizmo>,
    pub default_gizmo_parameters: TOptional<FGizmosParameters>,
    pub cached_gizmo_map: TMap<
        UPtr<crate::bindings::interactive_tools_framework::UInteractiveGizmoBuilder>,
        UPtr<crate::bindings::interactive_tools_framework::UInteractiveGizmo>,
    >,
}
pub struct UEditorGizmoRegistryCategoryEntry {
    pub gizmo_types: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UInteractiveGizmoBuilder>,
    >,
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
    pub transform_gizmo_builder: UPtr<
        crate::bindings::interactive_tools_framework::UInteractiveGizmoBuilder,
    >,
    pub registry: UPtr<UEditorInteractiveGizmoRegistry>,
}
pub struct UToolStackContext {
    pub ed_mode: TWeakObjectPtr<crate::bindings::unreal_ed::UEdMode>,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAxisRotateMode(pub u8);
impl EAxisRotateMode {
    pub const PULL: EAxisRotateMode = EAxisRotateMode(0);
    pub const ARC: EAxisRotateMode = EAxisRotateMode(1);
    pub const SCREEN_ARC: EAxisRotateMode = EAxisRotateMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoTransformMode(pub u8);
impl EGizmoTransformMode {
    pub const NONE: EGizmoTransformMode = EGizmoTransformMode(0);
    pub const TRANSLATE: EGizmoTransformMode = EGizmoTransformMode(1);
    pub const ROTATE: EGizmoTransformMode = EGizmoTransformMode(2);
    pub const SCALE: EGizmoTransformMode = EGizmoTransformMode(3);
    pub const MAX: EGizmoTransformMode = EGizmoTransformMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransformGizmoPartIdentifier(pub i32);
impl ETransformGizmoPartIdentifier {
    pub const DEFAULT: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(0);
    pub const TRANSLATE_ALL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        1,
    );
    pub const TRANSLATE_X_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        2,
    );
    pub const TRANSLATE_Y_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        3,
    );
    pub const TRANSLATE_Z_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        4,
    );
    pub const TRANSLATE_XY_PLANAR: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        5,
    );
    pub const TRANSLATE_YZ_PLANAR: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        6,
    );
    pub const TRANSLATE_XZ_PLANAR: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        7,
    );
    pub const TRANSLATE_SCREEN_SPACE: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        8,
    );
    pub const ROTATE_ALL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        9,
    );
    pub const ROTATE_X_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        10,
    );
    pub const ROTATE_Y_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        11,
    );
    pub const ROTATE_Z_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        12,
    );
    pub const ROTATE_SCREEN_SPACE: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        13,
    );
    pub const ROTATE_ARCBALL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        14,
    );
    pub const ROTATE_X_GIMBAL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        15,
    );
    pub const ROTATE_Y_GIMBAL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        16,
    );
    pub const ROTATE_Z_GIMBAL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        17,
    );
    pub const SCALE_ALL: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        18,
    );
    pub const SCALE_X_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        19,
    );
    pub const SCALE_Y_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        20,
    );
    pub const SCALE_Z_AXIS: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        21,
    );
    pub const SCALE_XY_PLANAR: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        22,
    );
    pub const SCALE_YZ_PLANAR: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        23,
    );
    pub const SCALE_XZ_PLANAR: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        24,
    );
    pub const SCALE_UNIFORM: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(
        25,
    );
    pub const MAX: ETransformGizmoPartIdentifier = ETransformGizmoPartIdentifier(26);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEditorGizmoCategory(pub u8);
impl EEditorGizmoCategory {
    pub const ACCESSORY: EEditorGizmoCategory = EEditorGizmoCategory(0);
    pub const PRIMARY: EEditorGizmoCategory = EEditorGizmoCategory(1);
}
