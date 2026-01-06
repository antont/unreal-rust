#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_CONTEXT_MENU_CONTEXT_GET_SCRIPT_HIT_PROXY_ELEMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_SET_EXACT_CAMERA_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_SET_CURRENT_LEVEL_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_SET_ALLOWS_CINEMATIC_CONTROL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_SAVE_CURRENT_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_SAVE_ALL_DIRTY_LEVELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_PILOT_LEVEL_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL_FROM_TEMPLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_LOAD_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_IS_IN_PLAY_IN_EDITOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_VIEWPORT_CONFIG_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_PILOT_LEVEL_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_EXACT_CAMERA_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_CURRENT_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_ALLOWS_CINEMATIC_CONTROL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_GET_ACTIVE_VIEWPORT_CONFIG_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EJECT_PILOT_LEVEL_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_VIEWPORT_REALTIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_GAME_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_END_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_BEGIN_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_PLAY_SIMULATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_INVALIDATE_VIEWPORTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_GET_GAME_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_EDITOR_SUBSYSTEM_BUILD_LIGHT_MAPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelEditorContextMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScriptHitProxyElement"),
            &raw mut U_LEVEL_EDITOR_CONTEXT_MENU_CONTEXT_GET_SCRIPT_HIT_PROXY_ELEMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExactCameraView"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_SET_EXACT_CAMERA_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentLevelByName"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_SET_CURRENT_LEVEL_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowsCinematicControl"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_SET_ALLOWS_CINEMATIC_CONTROL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveCurrentLevel"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_SAVE_CURRENT_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveAllDirtyLevels"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_SAVE_ALL_DIRTY_LEVELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PilotLevelActor"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_PILOT_LEVEL_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewLevelFromTemplate"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL_FROM_TEMPLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewLevel"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadLevel"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_LOAD_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInPlayInEditor"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_IS_IN_PLAY_IN_EDITOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewportConfigKeys"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_VIEWPORT_CONFIG_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionSet"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_SELECTION_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPilotLevelActor"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_PILOT_LEVEL_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExactCameraView"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_EXACT_CAMERA_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentLevel"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_CURRENT_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllowsCinematicControl"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_ALLOWS_CINEMATIC_CONTROL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveViewportConfigKey"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_GET_ACTIVE_VIEWPORT_CONFIG_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EjectPilotLevelActor"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EJECT_PILOT_LEVEL_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorSetViewportRealtime"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_VIEWPORT_REALTIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorSetGameView"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_GAME_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorRequestEndPlay"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_END_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorRequestBeginPlay"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_BEGIN_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorPlaySimulate"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_PLAY_SIMULATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorInvalidateViewports"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_INVALIDATE_VIEWPORTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorGetGameView"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_GET_GAME_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BuildLightMaps"),
            &raw mut U_LEVEL_EDITOR_SUBSYSTEM_BUILD_LIGHT_MAPS,
        );
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorCameraEditorState {
    __padding_end: [u8; 112],
}
impl ULevelEditorCameraEditorState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorCameraEditorState")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorMenuContext {
    __padding_end: [u8; 64],
}
impl ULevelEditorMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorMenuContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
impl ULevelEditorContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorContextMenuContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelViewportContext {
    __padding_end: [u8; 64],
}
impl ULevelViewportContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelViewportContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelViewportToolBarContext {
    __padding_end: [u8; 64],
}
impl ULevelViewportToolBarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelViewportToolBarContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UQuickActionMenuContext {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
}
impl UQuickActionMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuickActionMenuContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorSubsystem {
    __padding_end: [u8; 216],
}
impl ULevelEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULightEditorSubsystem {
    __padding_end: [u8; 72],
}
impl ULightEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULightEditorSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl ULevelEditorUISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorUISubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULegacyLevelViewportToolbarContext {
    __padding_end: [u8; 80],
}
impl ULegacyLevelViewportToolbarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyLevelViewportToolbarContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
