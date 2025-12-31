#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct ULevelEditorCameraEditorState {
    pub camera_location: crate::bindings::core_u_object::FVector,
    pub camera_rotation: crate::bindings::core_u_object::FRotator,
    pub camera_fov_angle: f32,
}
pub struct ULevelEditorMenuContext {}
pub struct ULevelEditorContextMenuContext {
    pub context_type: ELevelEditorMenuContext,
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
    pub cursor_world_location: crate::bindings::core_u_object::FVector,
    pub selected_components: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
    pub hit_proxy_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
}
pub struct ULevelViewportContext {}
pub struct ULevelViewportToolBarContext {}
pub struct UQuickActionMenuContext {
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
}
pub struct ULevelEditorSubsystem {
    pub on_pre_save_world: FLevelEditorSubsystem_OnPreSaveWorld,
    pub on_post_save_world: FLevelEditorSubsystem_OnPostSaveWorld,
    pub on_editor_camera_moved: FLevelEditorSubsystem_OnEditorCameraMoved,
    pub on_map_changed: FLevelEditorSubsystem_OnMapChanged,
    pub on_map_opened: FLevelEditorSubsystem_OnMapOpened,
}
pub struct ULightEditorSubsystem {}
pub struct ULevelEditorUISubsystem {}
pub struct ULegacyLevelViewportToolbarContext {}
pub struct FLevelEditorSubsystem_OnPreSaveWorld;
pub struct FLevelEditorSubsystem_OnPostSaveWorld;
pub struct FLevelEditorSubsystem_OnEditorCameraMoved;
pub struct FLevelEditorSubsystem_OnMapChanged;
pub struct FLevelEditorSubsystem_OnMapOpened;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelEditorMenuContext(pub u8);
impl ELevelEditorMenuContext {
    pub const VIEWPORT: ELevelEditorMenuContext = ELevelEditorMenuContext(0);
    pub const SCENE_OUTLINER: ELevelEditorMenuContext = ELevelEditorMenuContext(1);
    pub const MAIN_MENU: ELevelEditorMenuContext = ELevelEditorMenuContext(2);
}
