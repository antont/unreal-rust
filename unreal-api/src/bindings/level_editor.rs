#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct ULevelEditorCameraEditorState {
    __padding_end: [u8; 112],
}
impl ULevelEditorCameraEditorState {}
#[repr(C, align(8))]
pub struct ULevelEditorMenuContext {
    __padding_end: [u8; 64],
}
impl ULevelEditorMenuContext {}
#[repr(C, align(8))]
pub struct ULevelEditorContextMenuContext {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub context_type: ELevelEditorMenuContext,
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
    #[doc(hidden)]
    __padding_96: [u8; 16],
    pub cursor_world_location: crate::bindings::core_u_object::FVector,
    pub selected_components: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
    pub hit_proxy_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
}
impl ULevelEditorContextMenuContext {}
#[repr(C, align(8))]
pub struct ULevelViewportContext {
    __padding_end: [u8; 64],
}
impl ULevelViewportContext {}
#[repr(C, align(8))]
pub struct ULevelViewportToolBarContext {
    __padding_end: [u8; 64],
}
impl ULevelViewportToolBarContext {}
#[repr(C, align(8))]
pub struct UQuickActionMenuContext {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
}
impl UQuickActionMenuContext {}
#[repr(C, align(8))]
pub struct ULevelEditorSubsystem {
    __padding_end: [u8; 216],
}
impl ULevelEditorSubsystem {}
#[repr(C, align(8))]
pub struct ULightEditorSubsystem {
    __padding_end: [u8; 72],
}
impl ULightEditorSubsystem {}
#[repr(C, align(8))]
pub struct ULevelEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl ULevelEditorUISubsystem {}
#[repr(C, align(8))]
pub struct ULegacyLevelViewportToolbarContext {
    __padding_end: [u8; 80],
}
impl ULegacyLevelViewportToolbarContext {}
#[repr(C, align(8))]
pub struct FLevelEditorSubsystem_OnPreSaveWorld {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelEditorSubsystem_OnPostSaveWorld {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelEditorSubsystem_OnEditorCameraMoved {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelEditorSubsystem_OnMapChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelEditorSubsystem_OnMapOpened {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ELevelEditorMenuContext(pub u8);
impl ELevelEditorMenuContext {
    pub const VIEWPORT: ELevelEditorMenuContext = ELevelEditorMenuContext(0);
    pub const SCENE_OUTLINER: ELevelEditorMenuContext = ELevelEditorMenuContext(1);
    pub const MAIN_MENU: ELevelEditorMenuContext = ELevelEditorMenuContext(2);
}
