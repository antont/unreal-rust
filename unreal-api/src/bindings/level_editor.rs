#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_level_editor_context_menu_context_get_script_hit_proxy_element: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_set_exact_camera_view: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_set_current_level_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_set_allows_cinematic_control: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_save_current_level: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_save_all_dirty_levels: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_pilot_level_actor: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_new_level_from_template: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_new_level: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_load_level: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_is_in_play_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_viewport_config_keys: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_selection_set: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_pilot_level_actor: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_exact_camera_view: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_current_level: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_allows_cinematic_control: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_get_active_viewport_config_key: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_eject_pilot_level_actor: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_set_viewport_realtime: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_set_game_view: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_request_end_play: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_request_begin_play: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_play_simulate: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_invalidate_viewports: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_editor_get_game_view: *mut crate::ffi::UFunctionOpague,
    pub u_level_editor_subsystem_build_light_maps: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_level_editor_context_menu_context_get_script_hit_proxy_element: std::ptr::null_mut(),
            u_level_editor_subsystem_set_exact_camera_view: std::ptr::null_mut(),
            u_level_editor_subsystem_set_current_level_by_name: std::ptr::null_mut(),
            u_level_editor_subsystem_set_allows_cinematic_control: std::ptr::null_mut(),
            u_level_editor_subsystem_save_current_level: std::ptr::null_mut(),
            u_level_editor_subsystem_save_all_dirty_levels: std::ptr::null_mut(),
            u_level_editor_subsystem_pilot_level_actor: std::ptr::null_mut(),
            u_level_editor_subsystem_new_level_from_template: std::ptr::null_mut(),
            u_level_editor_subsystem_new_level: std::ptr::null_mut(),
            u_level_editor_subsystem_load_level: std::ptr::null_mut(),
            u_level_editor_subsystem_is_in_play_in_editor: std::ptr::null_mut(),
            u_level_editor_subsystem_get_viewport_config_keys: std::ptr::null_mut(),
            u_level_editor_subsystem_get_selection_set: std::ptr::null_mut(),
            u_level_editor_subsystem_get_pilot_level_actor: std::ptr::null_mut(),
            u_level_editor_subsystem_get_exact_camera_view: std::ptr::null_mut(),
            u_level_editor_subsystem_get_current_level: std::ptr::null_mut(),
            u_level_editor_subsystem_get_allows_cinematic_control: std::ptr::null_mut(),
            u_level_editor_subsystem_get_active_viewport_config_key: std::ptr::null_mut(),
            u_level_editor_subsystem_eject_pilot_level_actor: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_set_viewport_realtime: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_set_game_view: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_request_end_play: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_request_begin_play: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_play_simulate: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_invalidate_viewports: std::ptr::null_mut(),
            u_level_editor_subsystem_editor_get_game_view: std::ptr::null_mut(),
            u_level_editor_subsystem_build_light_maps: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULevelEditorContextMenuContext::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetScriptHitProxyElement"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_context_menu_context_get_script_hit_proxy_element,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULevelEditorSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetExactCameraView"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_set_exact_camera_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCurrentLevelByName"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_set_current_level_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAllowsCinematicControl"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_set_allows_cinematic_control,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SaveCurrentLevel"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_save_current_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SaveAllDirtyLevels"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_save_all_dirty_levels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PilotLevelActor"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_pilot_level_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NewLevelFromTemplate"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_new_level_from_template,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NewLevel"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_new_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadLevel"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_load_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInPlayInEditor"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_is_in_play_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetViewportConfigKeys"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_get_viewport_config_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionSet"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_get_selection_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPilotLevelActor"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_get_pilot_level_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetExactCameraView"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_get_exact_camera_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentLevel"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_get_current_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllowsCinematicControl"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_get_allows_cinematic_control,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveViewportConfigKey"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_get_active_viewport_config_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EjectPilotLevelActor"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_eject_pilot_level_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorSetViewportRealtime"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_set_viewport_realtime,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorSetGameView"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_editor_set_game_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorRequestEndPlay"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_editor_request_end_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorRequestBeginPlay"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_request_begin_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorPlaySimulate"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_editor_play_simulate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorInvalidateViewports"),
                &raw mut __FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_invalidate_viewports,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorGetGameView"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_editor_get_game_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BuildLightMaps"),
                &raw mut __FUNCTION_PTRS.u_level_editor_subsystem_build_light_maps,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorCameraEditorState {
    __padding_end: [u8; 112],
}
impl ULevelEditorCameraEditorState {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorCameraEditorState")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorCameraEditorState")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorMenuContext {
    __padding_end: [u8; 64],
}
impl ULevelEditorMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorContextMenuContext {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub context_type: ELevelEditorMenuContext,
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 16],
    pub cursor_world_location: crate::bindings::core_u_object::FVector,
    pub selected_components: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
    pub hit_proxy_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
}
impl ULevelEditorContextMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorContextMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorContextMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_context_menu_context_get_script_hit_proxy_element,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_context_menu_context_get_script_hit_proxy_element,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelViewportContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelViewportContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelViewportToolBarContext {
    __padding_end: [u8; 64],
}
impl ULevelViewportToolBarContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelViewportToolBarContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelViewportToolBarContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UQuickActionMenuContext {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub current_selection: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
}
impl UQuickActionMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuickActionMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuickActionMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorSubsystem {
    __padding_end: [u8; 216],
}
impl ULevelEditorSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorSubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_set_exact_camera_view,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_set_exact_camera_view,
                __buffer,
            )
        };
        std::mem::forget(b_exact_camera_view);
        std::mem::forget(viewport_config_key);
    }
    pub fn set_current_level_by_name(&mut self, level_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_set_current_level_by_name,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_set_current_level_by_name,
                __buffer,
            )
        };
        std::mem::forget(level_name);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_set_allows_cinematic_control,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_set_allows_cinematic_control,
                __buffer,
            )
        };
        std::mem::forget(b_allow);
        std::mem::forget(viewport_config_key);
    }
    pub fn save_current_level(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_save_current_level,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_save_current_level,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_save_all_dirty_levels,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_save_all_dirty_levels,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_pilot_level_actor,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_pilot_level_actor,
                __buffer,
            )
        };
        std::mem::forget(actor_to_pilot);
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_new_level_from_template,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_new_level_from_template,
                __buffer,
            )
        };
        std::mem::forget(asset_path);
        std::mem::forget(template_asset_path);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_new_level,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_new_level,
                __buffer,
            )
        };
        std::mem::forget(asset_path);
        std::mem::forget(b_is_partitioned_world);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_load_level,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_load_level,
                __buffer,
            )
        };
        std::mem::forget(asset_path);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_is_in_play_in_editor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_is_in_play_in_editor,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_viewport_config_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_viewport_config_keys,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_selection_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_selection_set,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_pilot_level_actor,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_pilot_level_actor,
                __buffer,
            )
        };
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_exact_camera_view,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_exact_camera_view,
                __buffer,
            )
        };
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_current_level,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_current_level,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_allows_cinematic_control,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_allows_cinematic_control,
                __buffer,
            )
        };
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_active_viewport_config_key,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_get_active_viewport_config_key,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_eject_pilot_level_actor,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_eject_pilot_level_actor,
                __buffer,
            )
        };
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_set_viewport_realtime,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_set_viewport_realtime,
                __buffer,
            )
        };
        std::mem::forget(b_in_realtime);
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_set_game_view,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_set_game_view,
                __buffer,
            )
        };
        std::mem::forget(b_game_view);
        std::mem::forget(viewport_config_key);
    }
    pub fn editor_request_end_play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_request_end_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_request_end_play,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_request_begin_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_request_begin_play,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_play_simulate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_play_simulate,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_invalidate_viewports,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_invalidate_viewports,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_get_game_view,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_editor_get_game_view,
                __buffer,
            )
        };
        std::mem::forget(viewport_config_key);
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_build_light_maps,
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
                crate::bindings::level_editor::__FUNCTION_PTRS
                    .u_level_editor_subsystem_build_light_maps,
                __buffer,
            )
        };
        std::mem::forget(quality);
        std::mem::forget(b_with_reflection_captures);
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULightEditorSubsystem {
    __padding_end: [u8; 72],
}
impl ULightEditorSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULightEditorSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULightEditorSubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl ULevelEditorUISubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorUISubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorUISubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULegacyLevelViewportToolbarContext {
    __padding_end: [u8; 80],
}
impl ULegacyLevelViewportToolbarContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyLevelViewportToolbarContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyLevelViewportToolbarContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
