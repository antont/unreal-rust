#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct ULevelEditorCameraEditorState {
    pub camera_location: FVector,
    pub camera_rotation: FRotator,
    pub camera_fov_angle: f32,
}
pub struct ULevelEditorMenuContext {}
pub struct ULevelEditorContextMenuContext {
    pub context_type: ELevelEditorMenuContext,
    pub current_selection: UPtr<UTypedElementSelectionSet>,
    pub cursor_world_location: FVector,
    pub selected_components: TArray<UPtr<UActorComponent>>,
    pub hit_proxy_actor: TWeakObjectPtr<AActor>,
}
pub struct ULevelViewportContext {}
pub struct ULevelViewportToolBarContext {}
pub struct UQuickActionMenuContext {
    pub current_selection: UPtr<UTypedElementSelectionSet>,
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
