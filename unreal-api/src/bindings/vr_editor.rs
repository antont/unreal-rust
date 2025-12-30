#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FVREditorFloatingUICreationContext {
    pub widget_class: TSubclassOf<UUserWidget>,
    pub panel_id: FName,
    pub parent_actor: UPtr<AActor>,
    pub panel_spawn_offset: FTransform,
    pub panel_size: FVector2D,
    pub panel_mesh: UPtr<UStaticMesh>,
    pub editor_ui_size: f32,
    pub b_hide_window_handles: bool,
    pub b_mask_out_widget_background: bool,
    pub b_no_close_button: bool,
}
#[repr(C, align(8))]
pub struct FVRButton {
    pub button_widget: UPtr<UVREditorWidgetComponent>,
}
pub struct UVREditorAssetContainer {
    pub dockable_window_close_sound: UPtr<USoundBase>,
    pub dockable_window_open_sound: UPtr<USoundBase>,
    pub dockable_window_drop_sound: UPtr<USoundBase>,
    pub dockable_window_drag_sound: UPtr<USoundBase>,
    pub drop_from_content_browser_sound: UPtr<USoundBase>,
    pub radial_menu_open_sound: UPtr<USoundBase>,
    pub radial_menu_close_sound: UPtr<USoundBase>,
    pub teleport_sound: UPtr<USoundBase>,
    pub button_press_sound: UPtr<USoundCue>,
    pub auto_scale_sound: UPtr<USoundBase>,
    pub generic_hmd_mesh: UPtr<UStaticMesh>,
    pub plane_mesh: UPtr<UStaticMesh>,
    pub cylinder_mesh: UPtr<UStaticMesh>,
    pub laser_pointer_start_mesh: UPtr<UStaticMesh>,
    pub laser_pointer_mesh: UPtr<UStaticMesh>,
    pub laser_pointer_end_mesh: UPtr<UStaticMesh>,
    pub laser_pointer_hover_mesh: UPtr<UStaticMesh>,
    pub vive_pre_controller_mesh: UPtr<UStaticMesh>,
    pub oculus_controller_mesh: UPtr<UStaticMesh>,
    pub generic_controller_mesh: UPtr<UStaticMesh>,
    pub teleport_root_mesh: UPtr<UStaticMesh>,
    pub window_mesh: UPtr<UStaticMesh>,
    pub window_selection_bar_mesh: UPtr<UStaticMesh>,
    pub window_close_button_mesh: UPtr<UStaticMesh>,
    pub radial_menu_main_mesh: UPtr<UStaticMesh>,
    pub radial_menu_pointer_mesh: UPtr<UStaticMesh>,
    pub pointer_cursor_mesh: UPtr<UStaticMesh>,
    pub line_segment_cylinder_mesh: UPtr<UStaticMesh>,
    pub joint_sphere_mesh: UPtr<UStaticMesh>,
    pub docking_button_mesh: UPtr<UStaticMesh>,
    pub grid_material: UPtr<UMaterialInterface>,
    pub laser_pointer_material: UPtr<UMaterialInterface>,
    pub laser_pointer_translucent_material: UPtr<UMaterialInterface>,
    pub world_movement_post_process_material: UPtr<UMaterial>,
    pub text_material: UPtr<UMaterialInterface>,
    pub vive_pre_controller_material: UPtr<UMaterialInterface>,
    pub oculus_controller_material: UPtr<UMaterialInterface>,
    pub teleport_material: UPtr<UMaterialInterface>,
    pub window_material: UPtr<UMaterialInterface>,
    pub window_translucent_material: UPtr<UMaterialInterface>,
    pub line_material: UPtr<UMaterial>,
    pub translucent_text_material: UPtr<UMaterialInterface>,
    pub widget_material: UPtr<UMaterialInterface>,
    pub camera_widget_material: UPtr<UMaterialInterface>,
    pub text_font: UPtr<UFont>,
}
pub struct UVREditorAutoScaler {
    pub vr_mode: UPtr<UVREditorMode>,
}
pub struct AVREditorTeleporter {
    pub vr_mode: UPtr<UVREditorMode>,
    pub teleport_direction_mesh_component: UPtr<UStaticMeshComponent>,
    pub hmd_mesh_component: UPtr<UStaticMeshComponent>,
    pub left_motion_controller_mesh_component: UPtr<UStaticMeshComponent>,
    pub right_motion_controller_mesh_component: UPtr<UStaticMeshComponent>,
    pub teleport_mid: UPtr<UMaterialInstanceDynamic>,
    pub interactor_trying_teleport: UPtr<UViewportInteractor>,
}
pub struct UVREditorBaseUserWidget {
    pub owner: TWeakObjectPtr<AVREditorFloatingUI>,
}
pub struct AVREditorBaseActor {
    pub vr_mode: UPtr<UVREditorMode>,
}
pub struct AVREditorFloatingUI {
    pub creation_context: FVREditorFloatingUICreationContext,
    pub user_widget: UPtr<UUserWidget>,
    pub widget_component: UPtr<UVREditorWidgetComponent>,
    pub window_mesh_component: UPtr<UStaticMeshComponent>,
    pub user_widget_class: TSubclassOf<UObject>,
}
pub struct AVREditorDockableWindow {
    pub dock_button_mesh_component: UPtr<UStaticMeshComponent>,
    pub selection_bar_mesh_component: UPtr<UStaticMeshComponent>,
    pub close_button_mesh_component: UPtr<UStaticMeshComponent>,
    pub dock_button_mid: UPtr<UMaterialInstanceDynamic>,
    pub selection_bar_mid: UPtr<UMaterialInstanceDynamic>,
    pub selection_bar_translucent_mid: UPtr<UMaterialInstanceDynamic>,
    pub close_button_mid: UPtr<UMaterialInstanceDynamic>,
    pub close_button_translucent_mid: UPtr<UMaterialInstanceDynamic>,
    pub drag_operation_component: UPtr<UViewportDragOperationComponent>,
}
pub struct AVREditorDockableCameraWindow {}
pub struct UDockableWindowDragOperation {}
pub struct AVREditorFloatingCameraUI {
    pub offset_from_camera: FVector,
    pub linked_actor: TWeakObjectPtr<AActor>,
}
pub struct AVREditorRadialFloatingUI {
    pub widget_components: TArray<UPtr<UVREditorWidgetComponent>>,
    pub window_mesh_component: UPtr<UStaticMeshComponent>,
    pub arrow_mesh_component: UPtr<UStaticMeshComponent>,
    pub central_widget_component: UPtr<UVREditorWidgetComponent>,
}
pub struct UVREditorUISystem {
    pub vr_mode: UPtr<UVREditorMode>,
    pub floating_u_is: TMap<FName, UPtr<AVREditorFloatingUI>>,
    pub preview_window_info: TMap<FName, UPtr<AActor>>,
    pub info_display_panel: UPtr<AVREditorFloatingUI>,
    pub quick_radial_menu: UPtr<AVREditorRadialFloatingUI>,
    pub dragging_ui: UPtr<AVREditorDockableWindow>,
    pub color_picker_ui: UPtr<AVREditorDockableWindow>,
    pub laser_interactor: UPtr<UVREditorInteractor>,
    pub ui_interactor: UPtr<UVREditorInteractor>,
    pub vr_buttons: TArray<FVRButton>,
    pub radial_menu_handler: UPtr<UVRRadialMenuHandler>,
}
pub struct UVRRadialMenuHandler {}
pub struct AVREditorAvatarActor {
    pub head_mesh_component: UPtr<UStaticMeshComponent>,
    pub world_movement_grid_mesh_component: UPtr<UStaticMeshComponent>,
    pub world_movement_grid_mid: UPtr<UMaterialInstanceDynamic>,
    pub world_movement_grid_opacity: f32,
    pub b_is_drawing_world_movement_post_process: bool,
    pub world_movement_post_process_material: UPtr<UMaterialInstanceDynamic>,
    pub scale_progress_mesh_component: UPtr<UStaticMeshComponent>,
    pub current_scale_progress_mesh_component: UPtr<UStaticMeshComponent>,
    pub user_scale_indicator_text: UPtr<UTextRenderComponent>,
    pub fixed_user_scale_mid: UPtr<UMaterialInstanceDynamic>,
    pub translucent_fixed_user_scale_mid: UPtr<UMaterialInstanceDynamic>,
    pub current_user_scale_mid: UPtr<UMaterialInstanceDynamic>,
    pub translucent_current_user_scale_mid: UPtr<UMaterialInstanceDynamic>,
    pub post_process_component: UPtr<UPostProcessComponent>,
    pub vr_mode: UPtr<UVREditorMode>,
}
pub struct UVREditorWidgetComponent {
    pub drawing_policy: EVREditorWidgetDrawingPolicy,
    pub b_is_hovering: bool,
    pub b_has_ever_drawn: bool,
}
pub struct UVREditorCameraWidgetComponent {}
pub struct AFloatingText {
    pub scene_component: UPtr<USceneComponent>,
    pub first_line_component: UPtr<UStaticMeshComponent>,
    pub joint_sphere_component: UPtr<UStaticMeshComponent>,
    pub second_line_component: UPtr<UStaticMeshComponent>,
    pub text_component: UPtr<UTextRenderComponent>,
    pub masked_text_material: UPtr<UMaterialInterface>,
    pub translucent_text_material: UPtr<UMaterialInterface>,
    pub line_material: UPtr<UMaterialInterface>,
    pub line_material_mid: UPtr<UMaterialInstanceDynamic>,
}
pub struct UVREditorInteractor {
    pub b_is_undo_redo_swipe_enabled: bool,
    pub motion_controller_component: UPtr<UMotionControllerComponent>,
    pub laser_motion_controller_component: UPtr<UMotionControllerComponent>,
    pub hand_mesh_component: UPtr<UStaticMeshComponent>,
    pub laser_spline_component: UPtr<USplineComponent>,
    pub laser_spline_mesh_components: TArray<UPtr<USplineMeshComponent>>,
    pub laser_pointer_mid: UPtr<UMaterialInstanceDynamic>,
    pub translucent_laser_pointer_mid: UPtr<UMaterialInstanceDynamic>,
    pub hover_mesh_component: UPtr<UStaticMeshComponent>,
    pub hover_point_light_component: UPtr<UPointLightComponent>,
    pub hand_mesh_mid: UPtr<UMaterialInstanceDynamic>,
    pub owning_avatar: UPtr<AActor>,
    pub controller_type: EControllerType,
    pub override_controller_type: EControllerType,
    pub controller_motion_source: FName,
    pub vr_mode: UPtr<UVREditorMode>,
}
pub struct UVREditorModeBase {}
pub struct UVREditorMode {
    pub avatar_actor: UPtr<AVREditorAvatarActor>,
    pub ui_system: UPtr<UVREditorUISystem>,
    pub teleport_actor: UPtr<AVREditorTeleporter>,
    pub auto_scaler_system: UPtr<UVREditorAutoScaler>,
    pub world_interaction: UPtr<UViewportWorldInteraction>,
    pub placement_system: UPtr<UVREditorPlacement>,
    pub interactors: TArray<UPtr<UVREditorInteractor>>,
    pub interactor_class: TSoftObjectPtr<UClass>,
    pub teleporter_class: TSoftObjectPtr<UClass>,
    pub asset_container: UPtr<UVREditorAssetContainer>,
}
pub struct UVREditorPlacement {
    pub vr_mode: UPtr<UVREditorMode>,
    pub viewport_world_interaction: UPtr<UViewportWorldInteraction>,
    pub floating_ui_asset_dragged_from: UPtr<UWidgetComponent>,
    pub placing_material_or_texture_asset: UPtr<UObject>,
}
pub struct UVRModeSettings {
    pub flags_56: u8,
    pub interactor_hand: EInteractorHand,
    pub flags_64: u8,
    pub ui_brightness: f32,
    pub gizmo_scale: f32,
    pub double_click_time: f32,
    pub trigger_pressed_threshold_vive: f32,
    pub trigger_pressed_threshold_rift: f32,
    pub mode_class: TSoftObjectPtr<UClass>,
}
pub struct UVRScoutingInteractor {
    pub flying_indicator_component: UPtr<UStaticMeshComponent>,
    pub editor_only_input_component: UPtr<UInputComponent>,
    pub b_receives_editor_input: bool,
}
