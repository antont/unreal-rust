#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FVREditorFloatingUICreationContext {
    pub widget_class: TSubclassOf<crate::bindings::umg::UUserWidget>,
    pub panel_id: FName,
    pub parent_actor: UPtr<crate::bindings::engine::AActor>,
    pub panel_spawn_offset: crate::bindings::core_u_object::FTransform,
    pub panel_size: crate::bindings::core_u_object::FVector2D,
    pub panel_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_ui_size: f32,
    pub b_hide_window_handles: bool,
    pub b_mask_out_widget_background: bool,
    pub b_no_close_button: bool,
    __padding_end: [u8; 1],
}
impl FVREditorFloatingUICreationContext {}
#[repr(C, align(8))]
pub struct UVREditorAssetContainer {
    __padding_end: [u8; 416],
}
impl UVREditorAssetContainer {}
#[repr(C, align(8))]
pub struct UVREditorAutoScaler {
    __padding_end: [u8; 56],
}
impl UVREditorAutoScaler {}
#[repr(C, align(8))]
pub struct AVREditorTeleporter {
    __padding_end: [u8; 1312],
}
impl AVREditorTeleporter {}
#[repr(C, align(8))]
pub struct UVREditorBaseUserWidget {
    __padding_end: [u8; 1296],
}
impl UVREditorBaseUserWidget {}
#[repr(C, align(16))]
pub struct AVREditorBaseActor {
    __padding_end: [u8; 1536],
}
impl AVREditorBaseActor {}
#[repr(C, align(16))]
pub struct AVREditorFloatingUI {
    __padding_end: [u8; 1808],
}
impl AVREditorFloatingUI {}
#[repr(C, align(16))]
pub struct AVREditorDockableWindow {
    __padding_end: [u8; 1920],
}
impl AVREditorDockableWindow {}
#[repr(C, align(16))]
pub struct AVREditorDockableCameraWindow {
    __padding_end: [u8; 1920],
}
impl AVREditorDockableCameraWindow {}
#[repr(C, align(16))]
pub struct UDockableWindowDragOperation {
    __padding_end: [u8; 288],
}
impl UDockableWindowDragOperation {}
#[repr(C, align(16))]
pub struct AVREditorFloatingCameraUI {
    __padding_end: [u8; 1840],
}
impl AVREditorFloatingCameraUI {}
#[repr(C, align(16))]
pub struct AVREditorRadialFloatingUI {
    __padding_end: [u8; 1712],
}
impl AVREditorRadialFloatingUI {}
#[repr(C, align(16))]
pub struct UVREditorUISystem {
    __padding_end: [u8; 544],
}
impl UVREditorUISystem {}
#[repr(C, align(8))]
pub struct UVRRadialMenuHandler {
    __padding_end: [u8; 216],
}
impl UVRRadialMenuHandler {}
#[repr(C, align(8))]
pub struct AVREditorAvatarActor {
    __padding_end: [u8; 1248],
}
impl AVREditorAvatarActor {}
#[repr(C, align(16))]
pub struct UVREditorWidgetComponent {
    __padding_end: [u8; 2032],
}
impl UVREditorWidgetComponent {}
#[repr(C, align(16))]
pub struct UVREditorCameraWidgetComponent {
    __padding_end: [u8; 2032],
}
impl UVREditorCameraWidgetComponent {}
#[repr(C, align(8))]
pub struct AFloatingText {
    __padding_end: [u8; 1208],
}
impl AFloatingText {}
#[repr(C, align(16))]
pub struct UVREditorInteractor {
    #[doc(hidden)]
    __padding_1968: [u8; 1968],
    pub hand_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    __padding_end: [u8; 600],
}
impl UVREditorInteractor {}
#[repr(C, align(8))]
pub struct UVREditorModeBase {
    __padding_end: [u8; 152],
}
impl UVREditorModeBase {}
#[repr(C, align(8))]
pub struct UVREditorMode {
    #[doc(hidden)]
    __padding_392: [u8; 392],
    pub interactor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub teleporter_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    __padding_end: [u8; 88],
}
impl UVREditorMode {}
#[repr(C, align(8))]
pub struct UVREditorPlacement {
    __padding_end: [u8; 80],
}
impl UVREditorPlacement {}
#[repr(C, align(8))]
pub struct UVRModeSettings {
    __padding_end: [u8; 136],
}
impl UVRModeSettings {}
#[repr(C, align(16))]
pub struct UVRScoutingInteractor {
    #[doc(hidden)]
    __padding_2576: [u8; 2576],
    pub flying_indicator_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    #[doc(hidden)]
    __padding_2592: [u8; 8],
    pub b_receives_editor_input: bool,
    __padding_end: [u8; 15],
}
impl UVRScoutingInteractor {}
#[repr(transparent)]
pub struct EControllerType(pub u8);
impl EControllerType {
    pub const LASER: EControllerType = EControllerType(0);
    pub const ASSISTING_LASER: EControllerType = EControllerType(1);
    pub const UI: EControllerType = EControllerType(2);
    pub const NAVIGATION: EControllerType = EControllerType(3);
    pub const UNKNOWN: EControllerType = EControllerType(4);
}
#[repr(transparent)]
pub struct EVREditorWidgetDrawingPolicy(pub u8);
impl EVREditorWidgetDrawingPolicy {
    pub const ALWAYS: EVREditorWidgetDrawingPolicy = EVREditorWidgetDrawingPolicy(0);
    pub const HOVERING: EVREditorWidgetDrawingPolicy = EVREditorWidgetDrawingPolicy(1);
}
#[repr(transparent)]
pub struct EInteractorHand(pub u8);
impl EInteractorHand {
    pub const RIGHT: EInteractorHand = EInteractorHand(0);
    pub const LEFT: EInteractorHand = EInteractorHand(1);
}
