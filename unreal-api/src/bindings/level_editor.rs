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
    pub fn get_hit_proxy_element(
        &mut self,
    ) -> crate::bindings::typed_element_framework::FScriptTypedElementHandle {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_CONTEXT_MENU_CONTEXT_GET_SCRIPT_HIT_PROXY_ELEMENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_CONTEXT_MENU_CONTEXT_GET_SCRIPT_HIT_PROXY_ELEMENT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                >()
                .read()
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
    pub fn set_exact_camera_view(
        &mut self,
        b_exact_camera_view: bool,
        viewport_config_key: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SET_EXACT_CAMERA_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_camera_view,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SET_EXACT_CAMERA_VIEW,
                __buffer,
            )
        };
    }
    pub fn set_current_level_by_name(&mut self, level_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SET_CURRENT_LEVEL_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SET_CURRENT_LEVEL_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_allows_cinematic_control(
        &mut self,
        b_allow: bool,
        viewport_config_key: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SET_ALLOWS_CINEMATIC_CONTROL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_allow, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SET_ALLOWS_CINEMATIC_CONTROL,
                __buffer,
            )
        };
    }
    pub fn save_current_level(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SAVE_CURRENT_LEVEL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SAVE_CURRENT_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn save_all_dirty_levels(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SAVE_ALL_DIRTY_LEVELS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_SAVE_ALL_DIRTY_LEVELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn pilot_level_actor(
        &mut self,
        actor_to_pilot: UPtr<crate::bindings::engine::AActor>,
        viewport_config_key: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_pilot,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
    }
    pub fn new_level_from_template(
        &mut self,
        asset_path: FString,
        template_asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL_FROM_TEMPLATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &template_asset_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL_FROM_TEMPLATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn new_level(
        &mut self,
        asset_path: FString,
        b_is_partitioned_world: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_partitioned_world,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_NEW_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn load_level(&mut self, asset_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_LOAD_LEVEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_LOAD_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_in_play_in_editor(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_IS_IN_PLAY_IN_EDITOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_IS_IN_PLAY_IN_EDITOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_viewport_config_keys(&mut self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_VIEWPORT_CONFIG_KEYS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_VIEWPORT_CONFIG_KEYS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_selection_set(
        &mut self,
    ) -> UPtr<crate::bindings::typed_element_runtime::UTypedElementSelectionSet> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_SELECTION_SET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<
                        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
                    >,
                >()
                .read()
        }
    }
    pub fn get_pilot_level_actor(
        &mut self,
        viewport_config_key: FName,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn get_exact_camera_view(&mut self, viewport_config_key: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_EXACT_CAMERA_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_EXACT_CAMERA_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_current_level(&mut self) -> UPtr<crate::bindings::engine::ULevel> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_CURRENT_LEVEL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_CURRENT_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>().read() }
    }
    pub fn get_allows_cinematic_control(&mut self, viewport_config_key: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_ALLOWS_CINEMATIC_CONTROL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_ALLOWS_CINEMATIC_CONTROL,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_active_viewport_config_key(&mut self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_ACTIVE_VIEWPORT_CONFIG_KEY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_GET_ACTIVE_VIEWPORT_CONFIG_KEY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn eject_pilot_level_actor(&mut self, viewport_config_key: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EJECT_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EJECT_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
    }
    pub fn editor_set_viewport_realtime(
        &mut self,
        b_in_realtime: bool,
        viewport_config_key: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_VIEWPORT_REALTIME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_realtime,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_VIEWPORT_REALTIME,
                __buffer,
            )
        };
    }
    pub fn editor_set_game_view(
        &mut self,
        b_game_view: bool,
        viewport_config_key: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_GAME_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_game_view,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_SET_GAME_VIEW,
                __buffer,
            )
        };
    }
    pub fn editor_request_end_play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_END_PLAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_END_PLAY,
                __buffer,
            )
        };
    }
    pub fn editor_request_begin_play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_BEGIN_PLAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_REQUEST_BEGIN_PLAY,
                __buffer,
            )
        };
    }
    pub fn editor_play_simulate(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_PLAY_SIMULATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_PLAY_SIMULATE,
                __buffer,
            )
        };
    }
    pub fn editor_invalidate_viewports(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_INVALIDATE_VIEWPORTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_INVALIDATE_VIEWPORTS,
                __buffer,
            )
        };
    }
    pub fn editor_get_game_view(&mut self, viewport_config_key: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_GET_GAME_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &viewport_config_key,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_EDITOR_GET_GAME_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn build_light_maps(
        &mut self,
        quality: crate::bindings::engine::ELightingBuildQuality,
        b_with_reflection_captures: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_BUILD_LIGHT_MAPS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &quality,
                __buffer.add(0).cast::<crate::bindings::engine::ELightingBuildQuality>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_with_reflection_captures,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::U_LEVEL_EDITOR_SUBSYSTEM_BUILD_LIGHT_MAPS,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
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
