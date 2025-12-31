#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGizmoVec2ParameterChange {
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub current_value: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(4))]
pub struct FGizmoElementColorAttribute {
    pub value: crate::bindings::core_u_object::FLinearColor,
    pub b_has_value: bool,
    pub b_overrides_child_state: bool,
}
#[repr(C, align(4))]
pub struct FGizmoElementMaterialAttribute {
    pub value: TWeakObjectPtr<crate::bindings::engine::UMaterialInterface>,
    pub b_overrides_child_state: bool,
}
#[repr(C, align(4))]
pub struct FGizmoElementMeshRenderStateAttributes {
    pub material: FGizmoElementMaterialAttribute,
    pub hover_material: FGizmoElementMaterialAttribute,
    pub interact_material: FGizmoElementMaterialAttribute,
    pub select_material: FGizmoElementMaterialAttribute,
    pub subdue_material: FGizmoElementMaterialAttribute,
    pub vertex_color: FGizmoElementColorAttribute,
    pub hover_vertex_color: FGizmoElementColorAttribute,
    pub interact_vertex_color: FGizmoElementColorAttribute,
    pub select_vertex_color: FGizmoElementColorAttribute,
    pub subdue_vertex_color: FGizmoElementColorAttribute,
}
#[repr(C, align(4))]
pub struct FGizmoElementLineRenderStateAttributes {
    pub line_color: FGizmoElementColorAttribute,
    pub hover_line_color: FGizmoElementColorAttribute,
    pub interact_line_color: FGizmoElementColorAttribute,
    pub select_line_color: FGizmoElementColorAttribute,
    pub subdue_line_color: FGizmoElementColorAttribute,
}
#[repr(C, align(8))]
pub struct FGizmoPerStateValueDouble {
    pub default: TOptional<f64>,
    pub hover: TOptional<f64>,
    pub interact: TOptional<f64>,
    pub select: TOptional<f64>,
    pub subdue: TOptional<f64>,
}
#[repr(C, align(4))]
pub struct FGizmoPerStateValueLinearColor {
    pub default: TOptional<crate::bindings::core_u_object::FLinearColor>,
    pub hover: TOptional<crate::bindings::core_u_object::FLinearColor>,
    pub interact: TOptional<crate::bindings::core_u_object::FLinearColor>,
    pub select: TOptional<crate::bindings::core_u_object::FLinearColor>,
    pub subdue: TOptional<crate::bindings::core_u_object::FLinearColor>,
}
#[repr(C, align(4))]
pub struct FGizmoFloatParameterChange {
    pub initial_value: f32,
    pub current_value: f32,
}
#[repr(C, align(8))]
pub struct FBrushStampData {}
#[repr(C, align(8))]
pub struct FBehaviorInfo {
    pub behavior: UPtr<UInputBehavior>,
}
#[repr(C, align(8))]
pub struct FInputRayHit {
    pub b_hit: bool,
    pub hit_depth: f64,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub b_has_hit_normal: bool,
    pub hit_identifier: i32,
    pub hit_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FDeviceButtonState {
    pub button: crate::bindings::input_core::FKey,
    pub b_pressed: bool,
    pub b_down: bool,
    pub b_released: bool,
    pub b_double_clicked: bool,
}
#[repr(C, align(8))]
pub struct FKeyboardInputDeviceState {
    pub active_key: FDeviceButtonState,
}
#[repr(C, align(8))]
pub struct FMouseInputDeviceState {
    pub left: FDeviceButtonState,
    pub middle: FDeviceButtonState,
    pub right: FDeviceButtonState,
    pub wheel_delta: f32,
    pub position2_d: crate::bindings::core_u_object::FVector2D,
    pub delta2_d: crate::bindings::core_u_object::FVector2D,
    pub world_ray: crate::bindings::core_u_object::FRay,
}
#[repr(C, align(8))]
pub struct FInputDeviceState {
    pub input_device: EInputDevices,
    pub b_shift_key_down: bool,
    pub b_alt_key_down: bool,
    pub b_ctrl_key_down: bool,
    pub b_cmd_key_down: bool,
    pub keyboard: FKeyboardInputDeviceState,
    pub mouse: FMouseInputDeviceState,
}
#[repr(C, align(8))]
pub struct FInputDeviceRay {
    pub world_ray: crate::bindings::core_u_object::FRay,
    pub b_has2_d: bool,
    pub screen_position: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FActiveGizmo {
    pub gizmo: UPtr<UInteractiveGizmo>,
}
pub struct UAssetBackedTarget {}
pub struct IAssetBackedTarget {}
pub struct UGizmoBaseComponentInterface {}
pub struct IGizmoBaseComponentInterface {}
pub struct UGizmoBaseComponent {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub hover_size_multiplier: f32,
    pub pixel_hit_distance_threshold: f32,
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
}
pub struct UInteractiveCommandArguments {}
pub struct UInteractiveCommandResult {}
pub struct UInteractiveCommand {}
pub struct UInteractiveGizmoBuilder {}
pub struct UToolContextTransactionProvider {}
pub struct IToolContextTransactionProvider {}
pub struct AInternalToolFrameworkActor {
    pub b_is_selectable_in_editor: bool,
}
pub struct UToolFrameworkComponent {}
pub struct IToolFrameworkComponent {}
pub struct UInteractiveToolCameraFocusAPI {}
pub struct IInteractiveToolCameraFocusAPI {}
pub struct UInteractiveToolNestedAcceptCancelAPI {}
pub struct IInteractiveToolNestedAcceptCancelAPI {}
pub struct UInteractiveToolShutdownQueryAPI {}
pub struct IInteractiveToolShutdownQueryAPI {}
pub struct UInteractiveToolExclusiveToolAPI {}
pub struct IInteractiveToolExclusiveToolAPI {}
pub struct UInteractiveToolEditorGizmoAPI {}
pub struct IInteractiveToolEditorGizmoAPI {}
pub struct UInteractiveToolManageGeometrySelectionAPI {}
pub struct IInteractiveToolManageGeometrySelectionAPI {}
pub struct UMaterialProvider {}
pub struct IMaterialProvider {}
pub struct UMeshDescriptionCommitter {}
pub struct IMeshDescriptionCommitter {}
pub struct UMeshDescriptionProvider {}
pub struct IMeshDescriptionProvider {}
pub struct UGizmoBaseVec2ParameterSource {}
pub struct UGizmoLocalVec2ParameterSource {
    pub value: crate::bindings::core_u_object::FVector2D,
    pub last_change: FGizmoVec2ParameterChange,
}
pub struct UPhysicsDataSource {}
pub struct IPhysicsDataSource {}
pub struct USceneComponentBackedTarget {}
pub struct ISceneComponentBackedTarget {}
pub struct UPrimitiveComponentBackedTarget {}
pub struct IPrimitiveComponentBackedTarget {}
pub struct USkeletalMeshBackedTarget {}
pub struct ISkeletalMeshBackedTarget {}
pub struct UStaticMeshBackedTarget {}
pub struct IStaticMeshBackedTarget {}
pub struct UInputBehavior {}
pub struct UAnyButtonInputBehavior {}
pub struct UClickDragInputBehavior {
    pub b_update_modifiers_during_drag: bool,
}
pub struct ULocalClickDragInputBehavior {}
pub struct USingleClickInputBehavior {
    pub hit_test_on_release: bool,
}
pub struct UDoubleClickInputBehavior {}
pub struct ULocalDoubleClickInputBehavior {}
pub struct UKeyAsModifierInputBehavior {}
pub struct UMouseButtonAsModifierInputBehavior {}
pub struct UKeyInputBehavior {}
pub struct UMouseHoverBehavior {}
pub struct ULocalMouseHoverBehavior {}
pub struct UMouseWheelInputBehavior {}
pub struct UMouseWheelModifierInputBehavior {}
pub struct UMultiClickSequenceInputBehavior {}
pub struct ULocalSingleClickInputBehavior {}
pub struct USingleClickOrDragInputBehavior {
    pub b_begin_drag_if_click_target_not_hit: bool,
    pub click_distance_threshold: f32,
}
pub struct USingleKeyCaptureBehavior {}
pub struct UTwoAxisPropertyEditInputBehavior {}
pub struct ULocalTwoAxisPropertyEditInputBehavior {}
pub struct UWidgetBaseBehavior {}
pub struct IWidgetBaseBehavior {}
pub struct UAxisAngleGizmoBuilder {}
pub struct UInteractiveGizmo {
    pub input_behaviors: UPtr<UInputBehaviorSet>,
}
pub struct UAxisAngleGizmo {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub angle_source: TScriptInterface<IGizmoFloatParameterSource>,
    pub hit_target: TScriptInterface<IGizmoClickTarget>,
    pub state_target: TScriptInterface<IGizmoStateTarget>,
    pub mouse_behavior: UPtr<UClickDragInputBehavior>,
    pub b_in_interaction: bool,
    pub rotation_origin: crate::bindings::core_u_object::FVector,
    pub rotation_axis: crate::bindings::core_u_object::FVector,
    pub rotation_plane_x: crate::bindings::core_u_object::FVector,
    pub rotation_plane_y: crate::bindings::core_u_object::FVector,
    pub interaction_start_point: crate::bindings::core_u_object::FVector,
    pub interaction_cur_point: crate::bindings::core_u_object::FVector,
    pub interaction_start_angle: f32,
    pub interaction_cur_angle: f32,
}
pub struct UAxisPositionGizmoBuilder {}
pub struct UAxisPositionGizmo {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub parameter_source: TScriptInterface<IGizmoFloatParameterSource>,
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
    pub hit_target: TScriptInterface<IGizmoClickTarget>,
    pub state_target: TScriptInterface<IGizmoStateTarget>,
    pub mouse_behavior: UPtr<UClickDragInputBehavior>,
    pub b_enable_signed_axis: bool,
    pub b_in_interaction: bool,
    pub interaction_origin: crate::bindings::core_u_object::FVector,
    pub interaction_axis: crate::bindings::core_u_object::FVector,
    pub interaction_start_point: crate::bindings::core_u_object::FVector,
    pub interaction_cur_point: crate::bindings::core_u_object::FVector,
    pub interaction_start_parameter: f32,
    pub interaction_cur_parameter: f32,
    pub parameter_sign: f32,
}
pub struct UGizmoConstantAxisSource {
    pub origin: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
}
pub struct UGizmoConstantFrameAxisSource {
    pub origin: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub tangent_x: crate::bindings::core_u_object::FVector,
    pub tangent_y: crate::bindings::core_u_object::FVector,
}
pub struct UGizmoWorldAxisSource {
    pub origin: crate::bindings::core_u_object::FVector,
    pub axis_index: i32,
}
pub struct UGizmoComponentAxisSource {
    pub component: UPtr<crate::bindings::engine::USceneComponent>,
    pub axis_index: i32,
    pub b_local_axes: bool,
}
pub struct UBrushStampIndicatorBuilder {}
pub struct UBrushStampIndicator {
    pub b_visible: bool,
    pub brush_radius: f32,
    pub brush_falloff: f32,
    pub brush_strength: f32,
    pub brush_position: crate::bindings::core_u_object::FVector,
    pub brush_normal: crate::bindings::core_u_object::FVector,
    pub b_draw_indicator_lines: bool,
    pub b_draw_radius_circle: bool,
    pub b_scale_normal_by_strength: bool,
    pub sample_step_count: i32,
    pub line_color: crate::bindings::core_u_object::FLinearColor,
    pub line_thickness: f32,
    pub b_depth_tested: bool,
    pub b_draw_secondary_lines: bool,
    pub secondary_line_thickness: f32,
    pub secondary_line_color: crate::bindings::core_u_object::FLinearColor,
    pub attached_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
}
pub struct AGizmoActor {}
pub struct ACombinedTransformGizmoActor {
    pub translate_x: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub translate_y: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub translate_z: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub translate_yz: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub translate_xz: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub translate_xy: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub rotate_x: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub rotate_y: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub rotate_z: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub rotation_sphere: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub free_rotate_handle: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub free_translate_handle: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub uniform_scale: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub axis_scale_x: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub axis_scale_y: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub axis_scale_z: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub plane_scale_yz: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub plane_scale_xz: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub plane_scale_xy: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub full_axis_scale_x: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub full_axis_scale_y: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub full_axis_scale_z: UPtr<crate::bindings::engine::UPrimitiveComponent>,
}
pub struct UCombinedTransformGizmoBuilder {}
pub struct UCombinedTransformGizmo {
    pub active_target: UPtr<UTransformProxy>,
    pub b_snap_to_world_grid: bool,
    pub b_grid_size_is_explicit: bool,
    pub explicit_grid_size: crate::bindings::core_u_object::FVector,
    pub b_rotation_grid_size_is_explicit: bool,
    pub explicit_rotation_grid_size: crate::bindings::core_u_object::FRotator,
    pub b_snap_to_world_rot_grid: bool,
    pub b_snap_to_scale_grid: bool,
    pub b_use_context_coordinate_system: bool,
    pub current_coordinate_system: EToolContextCoordinateSystem,
    pub b_use_context_gizmo_mode: bool,
    pub active_gizmo_mode: EToolContextTransformGizmoMode,
    pub active_components: TArray<UPtr<crate::bindings::engine::UPrimitiveComponent>>,
    pub active_gizmos: TArray<UPtr<UInteractiveGizmo>>,
    pub gizmo_actor: UPtr<ACombinedTransformGizmoActor>,
    pub camera_axis_source: UPtr<UGizmoConstantFrameAxisSource>,
    pub axis_x_source: UPtr<UGizmoComponentAxisSource>,
    pub axis_y_source: UPtr<UGizmoComponentAxisSource>,
    pub axis_z_source: UPtr<UGizmoComponentAxisSource>,
    pub unit_axis_x_source: UPtr<UGizmoComponentAxisSource>,
    pub unit_axis_y_source: UPtr<UGizmoComponentAxisSource>,
    pub unit_axis_z_source: UPtr<UGizmoComponentAxisSource>,
    pub state_target: UPtr<UGizmoTransformChangeStateTarget>,
}
pub struct UTransformProxy {
    pub b_rotate_per_object: bool,
    pub b_set_pivot_mode: bool,
    pub shared_transform: crate::bindings::core_u_object::FTransform,
    pub initial_shared_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UComponentBoundTransformProxy {}
pub struct UPlanePositionGizmo {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub parameter_source: TScriptInterface<IGizmoVec2ParameterSource>,
    pub hit_target: TScriptInterface<IGizmoClickTarget>,
    pub state_target: TScriptInterface<IGizmoStateTarget>,
    pub mouse_behavior: UPtr<UClickDragInputBehavior>,
    pub b_enable_signed_axis: bool,
    pub b_flip_x: bool,
    pub b_flip_y: bool,
    pub b_in_interaction: bool,
    pub interaction_origin: crate::bindings::core_u_object::FVector,
    pub interaction_normal: crate::bindings::core_u_object::FVector,
    pub interaction_axis_x: crate::bindings::core_u_object::FVector,
    pub interaction_axis_y: crate::bindings::core_u_object::FVector,
    pub interaction_start_point: crate::bindings::core_u_object::FVector,
    pub interaction_cur_point: crate::bindings::core_u_object::FVector,
    pub interaction_start_parameter: crate::bindings::core_u_object::FVector2D,
    pub interaction_cur_parameter: crate::bindings::core_u_object::FVector2D,
    pub parameter_signs: crate::bindings::core_u_object::FVector2D,
}
pub struct UFreePositionSubGizmo {
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
}
pub struct UFreeRotationSubGizmo {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub hit_target: TScriptInterface<IGizmoClickTarget>,
    pub state_target: TScriptInterface<IGizmoStateTarget>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
    pub mouse_behavior: UPtr<UClickDragInputBehavior>,
}
pub struct UGizmoArrowComponent {
    pub direction: crate::bindings::core_u_object::FVector,
    pub gap: f32,
    pub length: f32,
    pub thickness: f32,
}
pub struct UGizmoBoxComponent {
    pub origin: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub dimensions: crate::bindings::core_u_object::FVector,
    pub line_thickness: f32,
    pub b_remove_hidden_lines: bool,
    pub b_enable_axis_flip: bool,
}
pub struct UGizmoCircleComponent {
    pub normal: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub thickness: f32,
    pub num_sides: i32,
    pub b_view_aligned: bool,
    pub b_draw_full_circle: bool,
    pub b_only_allow_front_facing_hits: bool,
}
pub struct UGizmoElementBase {
    pub b_enabled: bool,
    pub b_enabled_for_perspective_projection: bool,
    pub b_enabled_for_orthographic_projection: bool,
    pub b_enabled_for_default_state: bool,
    pub b_enabled_for_hovering_state: bool,
    pub b_enabled_for_interacting_state: bool,
    pub b_enabled_for_selected_state: bool,
    pub b_enabled_for_subdued_state: bool,
    pub part_identifier: u32,
    pub mesh_render_attributes: FGizmoElementMeshRenderStateAttributes,
    pub element_state: EGizmoElementState,
    pub element_interaction_state: EGizmoElementInteractionState,
    pub view_dependent_type: EGizmoElementViewDependentType,
    pub view_dependent_axis: crate::bindings::core_u_object::FVector,
    pub view_dependent_angle_tol: f32,
    pub view_dependent_axial_max_cos_angle_tol: f32,
    pub view_dependent_planar_min_cos_angle_tol: f32,
    pub view_align_type: EGizmoElementViewAlignType,
    pub view_align_axis: crate::bindings::core_u_object::FVector,
    pub view_align_normal: crate::bindings::core_u_object::FVector,
    pub view_align_axial_angle_tol: f32,
    pub view_align_axial_max_cos_angle_tol: f32,
    pub pixel_hit_distance_threshold: f32,
    pub minimum_pixel_hit_distance_threshold: f32,
    pub hit_priority: i32,
    pub view_depth_offset: f32,
}
pub struct UGizmoElementLineBase {
    pub line_render_attributes: FGizmoElementLineRenderStateAttributes,
    pub line_thickness: f32,
    pub b_screen_space_line: bool,
    pub hover_line_thickness_multiplier: f32,
    pub interact_line_thickness_multiplier: f32,
    pub select_line_thickness_multiplier: f32,
    pub subdue_line_thickness_multiplier: f32,
}
pub struct UGizmoElementCircleBase {
    pub center: crate::bindings::core_u_object::FVector,
    pub axis0: crate::bindings::core_u_object::FVector,
    pub axis1: crate::bindings::core_u_object::FVector,
    pub radius: f64,
    pub radius_multiplier: FGizmoPerStateValueDouble,
    pub num_segments: i32,
    pub partial_type: EGizmoElementPartialType,
    pub partial_start_angle: f64,
    pub partial_end_angle: f64,
    pub partial_view_dependent_max_cos_tol: f64,
}
pub struct UGizmoElementArc {
    pub inner_radius: f64,
}
pub struct UGizmoElementGroupBase {
    pub b_constant_scale: bool,
    pub b_hit_owner: bool,
    pub elements: TArray<UPtr<UGizmoElementBase>>,
}
pub struct UGizmoElementArrow {
    pub cylinder_element: UPtr<UGizmoElementCylinder>,
    pub head_element: UPtr<UGizmoElementArrowHead>,
    pub base: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub side_direction: crate::bindings::core_u_object::FVector,
    pub body_length: f32,
    pub body_radius: f32,
    pub head_length: f32,
    pub head_radius: f32,
    pub num_sides: i32,
    pub b_end_caps: bool,
    pub head_type: EGizmoElementArrowHeadType,
    pub hit_mask: TWeakObjectPtr<UGizmoElementBase>,
}
pub struct UGizmoElementArrowHead {
    pub cone_element: UPtr<UGizmoElementCone>,
    pub box_element: UPtr<UGizmoElementBox>,
    pub sphere_element: UPtr<UGizmoElementSphere>,
    pub center: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub side_direction: crate::bindings::core_u_object::FVector,
    pub length: f32,
    pub radius: f32,
    pub num_sides: i32,
    pub ty: EGizmoElementArrowHeadType,
    pub hit_mask: TWeakObjectPtr<UGizmoElementBase>,
}
pub struct UGizmoElementBox {
    pub center: crate::bindings::core_u_object::FVector,
    pub dimensions: crate::bindings::core_u_object::FVector,
    pub up_direction: crate::bindings::core_u_object::FVector,
    pub side_direction: crate::bindings::core_u_object::FVector,
}
pub struct UGizmoElementCircle {
    pub b_draw_mesh: bool,
    pub b_draw_line: bool,
    pub b_hit_mesh: bool,
    pub b_hit_line: bool,
}
pub struct UGizmoElementCone {
    pub origin: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub height: f32,
    pub radius: f32,
    pub num_sides: i32,
    pub b_end_caps: bool,
}
pub struct UGizmoElementCylinder {
    pub base: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub height: f32,
    pub radius: f32,
    pub num_sides: i32,
    pub b_is_dashed: bool,
    pub dash_length: f32,
    pub dash_gap_length: f32,
}
pub struct UGizmoElementGroup {}
pub struct UGizmoElementHitTarget {
    pub gizmo_element: UPtr<UGizmoElementBase>,
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
    pub gizmo_transform_proxy: UPtr<UTransformProxy>,
}
pub struct UGizmoElementHitMultiTarget {
    pub gizmo_element: UPtr<UGizmoElementBase>,
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
    pub gizmo_transform_proxy: UPtr<UTransformProxy>,
}
pub struct UGizmoElementLineStrip {
    pub vertices: TArray<crate::bindings::core_u_object::FVector>,
    pub base: crate::bindings::core_u_object::FVector,
    pub up_direction: crate::bindings::core_u_object::FVector,
    pub side_direction: crate::bindings::core_u_object::FVector,
    pub b_draw_line_strip: bool,
}
pub struct UGizmoElementRectangle {
    pub center: crate::bindings::core_u_object::FVector,
    pub width: f32,
    pub height: f32,
    pub up_direction: crate::bindings::core_u_object::FVector,
    pub side_direction: crate::bindings::core_u_object::FVector,
    pub b_draw_mesh: bool,
    pub b_draw_line: bool,
    pub b_hit_mesh: bool,
    pub b_hit_line: bool,
}
pub struct UGizmoElementSphere {
    pub center: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub num_sides: i32,
}
pub struct UGizmoElementTorus {
    pub inner_radius: f64,
    pub inner_radius_multiplier: FGizmoPerStateValueDouble,
    pub num_inner_slices: i32,
    pub b_end_caps: bool,
}
pub struct UGizmoElementTriangleList {
    pub vertices: TArray<crate::bindings::core_u_object::FVector>,
    pub base: crate::bindings::core_u_object::FVector,
    pub up_direction: crate::bindings::core_u_object::FVector,
    pub side_direction: crate::bindings::core_u_object::FVector,
}
pub struct UGizmoTransformSource {}
pub struct IGizmoTransformSource {}
pub struct UGizmoAxisSource {}
pub struct IGizmoAxisSource {}
pub struct UGizmoClickTarget {}
pub struct IGizmoClickTarget {}
pub struct UGizmoClickMultiTarget {}
pub struct IGizmoClickMultiTarget {}
pub struct UGizmoRenderTarget {}
pub struct IGizmoRenderTarget {}
pub struct UGizmoRenderMultiTarget {}
pub struct IGizmoRenderMultiTarget {}
pub struct UGizmoStateTarget {}
pub struct IGizmoStateTarget {}
pub struct UGizmoFloatParameterSource {}
pub struct IGizmoFloatParameterSource {}
pub struct UGizmoVec2ParameterSource {}
pub struct IGizmoVec2ParameterSource {}
pub struct UGizmoLineHandleComponent {
    pub normal: crate::bindings::core_u_object::FVector,
    pub handle_size: f32,
    pub thickness: f32,
    pub direction: crate::bindings::core_u_object::FVector,
    pub length: f32,
    pub b_image_scale: bool,
}
pub struct UGizmoRectangleComponent {
    pub direction_x: crate::bindings::core_u_object::FVector,
    pub direction_y: crate::bindings::core_u_object::FVector,
    pub b_orient_y_according_to_camera: bool,
    pub offset_x: f32,
    pub offset_y: f32,
    pub length_x: f32,
    pub length_y: f32,
    pub thickness: f32,
    pub segment_flags: u8,
}
pub struct USimpleLambdaInteractiveGizmoBuilder {}
pub struct UGizmoViewContext {}
pub struct UGizmoLambdaHitTarget {}
pub struct UGizmoComponentHitTarget {
    pub component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
}
pub struct AIntervalGizmoActor {
    pub up_interval_component: UPtr<UGizmoLineHandleComponent>,
    pub down_interval_component: UPtr<UGizmoLineHandleComponent>,
    pub forward_interval_component: UPtr<UGizmoLineHandleComponent>,
    pub backward_interval_component: UPtr<UGizmoLineHandleComponent>,
    pub right_interval_component: UPtr<UGizmoLineHandleComponent>,
    pub left_interval_component: UPtr<UGizmoLineHandleComponent>,
}
pub struct UIntervalGizmoBuilder {}
pub struct UIntervalGizmo {
    pub state_target: UPtr<UGizmoTransformChangeStateTarget>,
    pub world: UPtr<crate::bindings::engine::UWorld>,
    pub gizmo_actor: UPtr<AIntervalGizmoActor>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub active_components: TArray<UPtr<crate::bindings::engine::UPrimitiveComponent>>,
    pub active_gizmos: TArray<UPtr<UInteractiveGizmo>>,
    pub up_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub down_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub forward_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub backward_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub right_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub left_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub axis_x_source: UPtr<UGizmoComponentAxisSource>,
    pub axis_y_source: UPtr<UGizmoComponentAxisSource>,
    pub axis_z_source: UPtr<UGizmoComponentAxisSource>,
}
pub struct UGizmoBaseFloatParameterSource {}
pub struct UGizmoAxisIntervalParameterSource {
    pub float_parameter_source: TScriptInterface<IGizmoFloatParameterSource>,
    pub min_parameter: f32,
    pub max_parameter: f32,
}
pub struct UGizmoLocalFloatParameterSource {
    pub value: f32,
    pub last_change: FGizmoFloatParameterChange,
}
pub struct UGizmoAxisTranslationParameterSource {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub parameter: f32,
    pub last_change: FGizmoFloatParameterChange,
    pub cur_translation_axis: crate::bindings::core_u_object::FVector,
    pub cur_translation_origin: crate::bindings::core_u_object::FVector,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UGizmoPlaneTranslationParameterSource {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub parameter: crate::bindings::core_u_object::FVector2D,
    pub last_change: FGizmoVec2ParameterChange,
    pub cur_translation_origin: crate::bindings::core_u_object::FVector,
    pub cur_translation_normal: crate::bindings::core_u_object::FVector,
    pub cur_translation_axis_x: crate::bindings::core_u_object::FVector,
    pub cur_translation_axis_y: crate::bindings::core_u_object::FVector,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UGizmoAxisRotationParameterSource {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub angle: f32,
    pub last_change: FGizmoFloatParameterChange,
    pub cur_rotation_axis: crate::bindings::core_u_object::FVector,
    pub cur_rotation_origin: crate::bindings::core_u_object::FVector,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UGizmoUniformScaleParameterSource {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub scale_multiplier: f32,
    pub parameter: crate::bindings::core_u_object::FVector2D,
    pub last_change: FGizmoVec2ParameterChange,
    pub cur_scale_origin: crate::bindings::core_u_object::FVector,
    pub cur_scale_normal: crate::bindings::core_u_object::FVector,
    pub cur_scale_axis_x: crate::bindings::core_u_object::FVector,
    pub cur_scale_axis_y: crate::bindings::core_u_object::FVector,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UGizmoAxisScaleParameterSource {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub scale_multiplier: f32,
    pub b_clamp_to_zero: bool,
    pub parameter: f32,
    pub last_change: FGizmoFloatParameterChange,
    pub cur_scale_axis: crate::bindings::core_u_object::FVector,
    pub cur_scale_origin: crate::bindings::core_u_object::FVector,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UGizmoPlaneScaleParameterSource {
    pub axis_source: TScriptInterface<IGizmoAxisSource>,
    pub transform_source: TScriptInterface<IGizmoTransformSource>,
    pub scale_multiplier: f32,
    pub b_use_equal_scaling: bool,
    pub b_clamp_to_zero: bool,
    pub parameter: crate::bindings::core_u_object::FVector2D,
    pub last_change: FGizmoVec2ParameterChange,
    pub cur_scale_origin: crate::bindings::core_u_object::FVector,
    pub cur_scale_normal: crate::bindings::core_u_object::FVector,
    pub cur_scale_axis_x: crate::bindings::core_u_object::FVector,
    pub cur_scale_axis_y: crate::bindings::core_u_object::FVector,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UPlanePositionGizmoBuilder {}
pub struct URepositionableTransformGizmoBuilder {}
pub struct URepositionableTransformGizmo {
    pub reposition_state_target: UPtr<UGizmoTransformChangeStateTarget>,
}
pub struct UScalableSphereGizmoBuilder {}
pub struct UScalableSphereGizmo {
    pub hit_error_threshold: f32,
    pub transaction_description: FText,
    pub radius: f32,
    pub b_is_hovering: bool,
    pub b_is_dragging: bool,
    pub active_target: UPtr<UTransformProxy>,
    pub active_axis: crate::bindings::core_u_object::FVector,
    pub drag_start_world_position: crate::bindings::core_u_object::FVector,
    pub drag_current_position_projected: crate::bindings::core_u_object::FVector,
    pub interaction_start_parameter: f32,
}
pub struct UScalableSphereGizmoInputBehavior {}
pub struct USimpleSingleClickGizmo {
    pub hit_target: TScriptInterface<IGizmoClickTarget>,
    pub click_behavior: UPtr<USingleClickInputBehavior>,
}
pub struct UGizmoNilStateTarget {}
pub struct UGizmoLambdaStateTarget {}
pub struct UGizmoObjectModifyStateTarget {
    pub transaction_manager: TScriptInterface<IToolContextTransactionProvider>,
}
pub struct UGizmoTransformChangeStateTarget {
    pub transaction_manager: TScriptInterface<IToolContextTransactionProvider>,
}
pub struct UCombinedTransformGizmoContextObject {}
pub struct UGizmoBaseTransformSource {}
pub struct UGizmoComponentWorldTransformSource {
    pub component: UPtr<crate::bindings::engine::USceneComponent>,
    pub b_modify_component_on_transform: bool,
}
pub struct UGizmoScaledTransformSource {
    pub child_transform_source: TScriptInterface<IGizmoTransformSource>,
}
pub struct UGizmoTransformProxyTransformSource {
    pub proxy: UPtr<UTransformProxy>,
}
pub struct UGizmoScaledAndUnscaledTransformSources {
    pub scaled_transform_source: TScriptInterface<IGizmoTransformSource>,
    pub unscaled_transform_source: TScriptInterface<IGizmoTransformSource>,
}
pub struct UViewAdjustedStaticMeshGizmoComponent {
    pub gizmo_view_context: UPtr<UGizmoViewContext>,
    pub substitute_interaction_component: UPtr<
        crate::bindings::engine::UPrimitiveComponent,
    >,
    pub hover_override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UInteractiveToolPropertySet {
    pub cached_properties_map: TMap<FString, UPtr<UInteractiveToolPropertySet>>,
    pub b_is_property_set_enabled: bool,
}
pub struct UBrushBaseProperties {
    pub brush_size: f32,
    pub b_specify_radius: bool,
    pub brush_radius: f32,
    pub b_tool_supports_pressure_sensitivity: bool,
    pub b_enable_pressure_sensitivity: bool,
    pub brush_strength: f32,
    pub brush_falloff_amount: f32,
    pub b_hit_back_faces: bool,
    pub b_show_strength: bool,
    pub b_show_falloff: bool,
    pub b_show_hit_back_faces: bool,
}
pub struct UBrushAdjusterInputBehavior {}
pub struct UInteractiveTool {
    pub input_behaviors: UPtr<UInputBehaviorSet>,
    pub tool_property_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct USingleSelectionTool {
    pub target: UPtr<UToolTarget>,
}
pub struct UMeshSurfacePointTool {
    pub target_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
}
pub struct UBaseBrushTool {
    pub brush_properties: UPtr<UBrushBaseProperties>,
    pub b_in_brush_stroke: bool,
    pub world_to_local_scale: f32,
    pub last_brush_stamp: FBrushStampData,
    pub property_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub brush_stamp_indicator: UPtr<UBrushStampIndicator>,
}
pub struct UInteractiveToolBuilder {}
pub struct UClickDragToolBuilder {}
pub struct UClickDragTool {}
pub struct UInteractiveToolWithToolTargetsBuilder {}
pub struct UMeshSurfacePointToolBuilder {}
pub struct USingleClickToolBuilder {}
pub struct USingleClickTool {}
pub struct UContextObjectStore {
    pub context_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UInputBehaviorSet {
    pub behaviors: TArray<FBehaviorInfo>,
}
pub struct UInputBehaviorSource {}
pub struct IInputBehaviorSource {}
pub struct ULocalInputBehaviorSource {}
pub struct UInputRouter {
    pub b_auto_invalidate_on_hover: bool,
    pub b_auto_invalidate_on_capture: bool,
    pub active_input_behaviors: UPtr<UInputBehaviorSet>,
}
pub struct UInteractionMechanic {}
pub struct UInteractiveGizmoManager {
    pub active_gizmos: TArray<FActiveGizmo>,
    pub gizmo_builders: TMap<FString, UPtr<UInteractiveGizmoBuilder>>,
}
pub struct UInteractiveToolManager {
    pub active_left_tool: UPtr<UInteractiveTool>,
    pub active_right_tool: UPtr<UInteractiveTool>,
    pub tool_builders: TMap<FString, UPtr<UInteractiveToolBuilder>>,
}
pub struct UInteractiveToolsContext {
    pub input_router: UPtr<UInputRouter>,
    pub target_manager: UPtr<UToolTargetManager>,
    pub tool_manager: UPtr<UInteractiveToolManager>,
    pub gizmo_manager: UPtr<UInteractiveGizmoManager>,
    pub context_object_store: UPtr<UContextObjectStore>,
    pub tool_manager_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
}
pub struct UMultiSelectionTool {
    pub targets: TArray<UPtr<UToolTarget>>,
}
pub struct USceneSnappingManager {}
pub struct USelectionSet {}
pub struct UMeshSelectionSet {
    pub vertices: TArray<i32>,
    pub edges: TArray<i32>,
    pub faces: TArray<i32>,
    pub groups: TArray<i32>,
}
pub struct UToolsContextCursorAPI {}
pub struct UToolTargetManager {
    pub factories: TArray<UPtr<UToolTargetFactory>>,
}
pub struct UToolTarget {}
pub struct UPrimitiveComponentToolTarget {}
pub struct UToolTargetFactory {}
pub struct UPrimitiveComponentToolTargetFactory {}
pub struct USceneComponentToolTarget {}
pub struct USceneComponentToolTargetFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputDevices(pub i32);
impl EInputDevices {
    pub const NONE: EInputDevices = EInputDevices(0);
    pub const KEYBOARD: EInputDevices = EInputDevices(1);
    pub const MOUSE: EInputDevices = EInputDevices(2);
    pub const GAMEPAD: EInputDevices = EInputDevices(4);
    pub const OCULUS_TOUCH: EInputDevices = EInputDevices(8);
    pub const HTC_VIVE_WANDS: EInputDevices = EInputDevices(16);
    pub const ANY_SPATIAL_DEVICE: EInputDevices = EInputDevices(24);
    pub const TABLET_FINGERS: EInputDevices = EInputDevices(1024);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolContextCoordinateSystem(pub u8);
impl EToolContextCoordinateSystem {
    pub const WORLD: EToolContextCoordinateSystem = EToolContextCoordinateSystem(0);
    pub const LOCAL: EToolContextCoordinateSystem = EToolContextCoordinateSystem(1);
    pub const SCREEN: EToolContextCoordinateSystem = EToolContextCoordinateSystem(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolContextTransformGizmoMode(pub u8);
impl EToolContextTransformGizmoMode {
    pub const NO_GIZMO: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        0,
    );
    pub const TRANSLATION: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        1,
    );
    pub const ROTATION: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        2,
    );
    pub const SCALE: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(3);
    pub const COMBINED: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        8,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoElementState(pub u8);
impl EGizmoElementState {
    pub const NONE: EGizmoElementState = EGizmoElementState(0);
    pub const VISIBLE: EGizmoElementState = EGizmoElementState(2);
    pub const HITTABLE: EGizmoElementState = EGizmoElementState(4);
    pub const VISIBLE_AND_HITTABLE: EGizmoElementState = EGizmoElementState(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoElementInteractionState(pub i32);
impl EGizmoElementInteractionState {
    pub const NONE: EGizmoElementInteractionState = EGizmoElementInteractionState(0);
    pub const HOVERING: EGizmoElementInteractionState = EGizmoElementInteractionState(1);
    pub const INTERACTING: EGizmoElementInteractionState = EGizmoElementInteractionState(
        2,
    );
    pub const SELECTED: EGizmoElementInteractionState = EGizmoElementInteractionState(3);
    pub const SUBDUED: EGizmoElementInteractionState = EGizmoElementInteractionState(4);
    pub const MAX: EGizmoElementInteractionState = EGizmoElementInteractionState(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoElementViewDependentType(pub i32);
impl EGizmoElementViewDependentType {
    pub const NONE: EGizmoElementViewDependentType = EGizmoElementViewDependentType(0);
    pub const AXIS: EGizmoElementViewDependentType = EGizmoElementViewDependentType(1);
    pub const PLANE: EGizmoElementViewDependentType = EGizmoElementViewDependentType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoElementViewAlignType(pub i32);
impl EGizmoElementViewAlignType {
    pub const NONE: EGizmoElementViewAlignType = EGizmoElementViewAlignType(0);
    pub const POINT_ONLY: EGizmoElementViewAlignType = EGizmoElementViewAlignType(1);
    pub const POINT_EYE: EGizmoElementViewAlignType = EGizmoElementViewAlignType(2);
    pub const POINT_SCREEN: EGizmoElementViewAlignType = EGizmoElementViewAlignType(3);
    pub const AXIAL: EGizmoElementViewAlignType = EGizmoElementViewAlignType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoElementPartialType(pub i32);
impl EGizmoElementPartialType {
    pub const NONE: EGizmoElementPartialType = EGizmoElementPartialType(0);
    pub const PARTIAL: EGizmoElementPartialType = EGizmoElementPartialType(1);
    pub const PARTIAL_VIEW_DEPENDENT: EGizmoElementPartialType = EGizmoElementPartialType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGizmoElementArrowHeadType(pub i32);
impl EGizmoElementArrowHeadType {
    pub const NONE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(0);
    pub const CONE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(1);
    pub const CUBE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(2);
    pub const SPHERE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(3);
}
