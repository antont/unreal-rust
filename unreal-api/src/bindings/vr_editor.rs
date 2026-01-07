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
pub static mut AVR_EDITOR_TELEPORTER_TELEPORT_DONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_STOP_AIMING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_START_TELEPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_START_AIMING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_SHUTDOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_SET_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_SET_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_IS_TELEPORTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_IS_AIMING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_INIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_GET_VR_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_GET_SLIDE_DELTA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_GET_INTERACTOR_TRYING_TELEPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AVR_EDITOR_TELEPORTER_DO_TELEPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_UPDATE_HAND_MESH_RELATIVE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_TRY_OVERRIDE_CONTROLLER_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_SETUP_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_SET_FORCE_SHOW_LASER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_SET_FORCE_LASER_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_SET_CONTROLLER_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_SET_CONTROLLER_HAND_SIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_REPLACE_HAND_MESH_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_IS_TOUCHING_TRACKPAD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_IS_HOVERING_OVER_UI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_IS_CLICKING_ON_UI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_INIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_TRACKPAD_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_TELEPORT_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_SLIDE_DELTA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_SELECT_AND_MOVE_TRIGGER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_MOTION_CONTROLLER_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_LAST_TRACKPAD_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_LASER_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_LASER_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_HMD_DEVICE_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_CONTROLLER_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_CONTROLLER_SIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_INTERACTOR_GET_CONTROLLER_HAND_SIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_MODE_SET_GAME_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_MODE_IS_IN_GAME_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_EDITOR_MODE_GET_WORLD_SCALE_FACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_SCOUTING_INTERACTOR_SET_RECEIVES_EDITOR_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_SCOUTING_INTERACTOR_SET_GIZMO_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_SCOUTING_INTERACTOR_GET_SELECTED_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_SCOUTING_INTERACTOR_GET_RECEIVES_EDITOR_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_SCOUTING_INTERACTOR_GET_INPUT_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UVR_SCOUTING_INTERACTOR_GET_GIZMO_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AVREditorTeleporter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TeleportDone"),
            &raw mut AVR_EDITOR_TELEPORTER_TELEPORT_DONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAiming"),
            &raw mut AVR_EDITOR_TELEPORTER_STOP_AIMING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartTeleport"),
            &raw mut AVR_EDITOR_TELEPORTER_START_TELEPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartAiming"),
            &raw mut AVR_EDITOR_TELEPORTER_START_AIMING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Shutdown"),
            &raw mut AVR_EDITOR_TELEPORTER_SHUTDOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVisibility"),
            &raw mut AVR_EDITOR_TELEPORTER_SET_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColor"),
            &raw mut AVR_EDITOR_TELEPORTER_SET_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTeleporting"),
            &raw mut AVR_EDITOR_TELEPORTER_IS_TELEPORTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAiming"),
            &raw mut AVR_EDITOR_TELEPORTER_IS_AIMING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Init"),
            &raw mut AVR_EDITOR_TELEPORTER_INIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVRMode"),
            &raw mut AVR_EDITOR_TELEPORTER_GET_VR_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlideDelta"),
            &raw mut AVR_EDITOR_TELEPORTER_GET_SLIDE_DELTA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInteractorTryingTeleport"),
            &raw mut AVR_EDITOR_TELEPORTER_GET_INTERACTOR_TRYING_TELEPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoTeleport"),
            &raw mut AVR_EDITOR_TELEPORTER_DO_TELEPORT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVREditorInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateHandMeshRelativeTransform"),
            &raw mut UVR_EDITOR_INTERACTOR_UPDATE_HAND_MESH_RELATIVE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryOverrideControllerType"),
            &raw mut UVR_EDITOR_INTERACTOR_TRY_OVERRIDE_CONTROLLER_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupComponent"),
            &raw mut UVR_EDITOR_INTERACTOR_SETUP_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForceShowLaser"),
            &raw mut UVR_EDITOR_INTERACTOR_SET_FORCE_SHOW_LASER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForceLaserColor"),
            &raw mut UVR_EDITOR_INTERACTOR_SET_FORCE_LASER_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControllerType"),
            &raw mut UVR_EDITOR_INTERACTOR_SET_CONTROLLER_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControllerHandSide"),
            &raw mut UVR_EDITOR_INTERACTOR_SET_CONTROLLER_HAND_SIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceHandMeshComponent"),
            &raw mut UVR_EDITOR_INTERACTOR_REPLACE_HAND_MESH_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTouchingTrackpad"),
            &raw mut UVR_EDITOR_INTERACTOR_IS_TOUCHING_TRACKPAD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsHoveringOverUI"),
            &raw mut UVR_EDITOR_INTERACTOR_IS_HOVERING_OVER_UI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsClickingOnUI"),
            &raw mut UVR_EDITOR_INTERACTOR_IS_CLICKING_ON_UI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Init"),
            &raw mut UVR_EDITOR_INTERACTOR_INIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackpadPosition"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_TRACKPAD_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTeleportActor"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_TELEPORT_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlideDelta"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_SLIDE_DELTA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectAndMoveTriggerValue"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_SELECT_AND_MOVE_TRIGGER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMotionControllerComponent"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_MOTION_CONTROLLER_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastTrackpadPosition"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_LAST_TRACKPAD_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLaserStart"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_LASER_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLaserEnd"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_LASER_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHMDDeviceType"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_HMD_DEVICE_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControllerType"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_CONTROLLER_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControllerSide"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_CONTROLLER_SIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControllerHandSide"),
            &raw mut UVR_EDITOR_INTERACTOR_GET_CONTROLLER_HAND_SIDE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVREditorMode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGameView"),
            &raw mut UVR_EDITOR_MODE_SET_GAME_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInGameView"),
            &raw mut UVR_EDITOR_MODE_IS_IN_GAME_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorldScaleFactor"),
            &raw mut UVR_EDITOR_MODE_GET_WORLD_SCALE_FACTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVRScoutingInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReceivesEditorInput"),
            &raw mut UVR_SCOUTING_INTERACTOR_SET_RECEIVES_EDITOR_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGizmoMode"),
            &raw mut UVR_SCOUTING_INTERACTOR_SET_GIZMO_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedActors"),
            &raw mut UVR_SCOUTING_INTERACTOR_GET_SELECTED_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReceivesEditorInput"),
            &raw mut UVR_SCOUTING_INTERACTOR_GET_RECEIVES_EDITOR_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputComponent"),
            &raw mut UVR_SCOUTING_INTERACTOR_GET_INPUT_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGizmoMode"),
            &raw mut UVR_SCOUTING_INTERACTOR_GET_GIZMO_MODE,
        );
    }
}
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
}
impl FVREditorFloatingUICreationContext {}
#[repr(C, align(8))]
pub struct UVREditorAssetContainer {
    __padding_end: [u8; 416],
}
impl UVREditorAssetContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorAssetContainer")
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
pub struct UVREditorAutoScaler {
    __padding_end: [u8; 56],
}
impl UVREditorAutoScaler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorAutoScaler")
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
pub struct AVREditorTeleporter {
    __padding_end: [u8; 1312],
}
impl AVREditorTeleporter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorTeleporter")
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
    pub fn stop_aiming(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_STOP_AIMING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_STOP_AIMING,
                __buffer,
            )
        };
    }
    pub fn start_aiming(
        &mut self,
        interactor: UPtr<crate::bindings::viewport_interaction::UViewportInteractor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_START_AIMING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interactor,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::viewport_interaction::UViewportInteractor>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_START_AIMING,
                __buffer,
            )
        };
    }
    pub fn set_visibility(&mut self, b_visible: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_SET_VISIBILITY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_visible, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_SET_VISIBILITY,
                __buffer,
            )
        };
    }
    pub fn set_color(&mut self, color: &crate::bindings::core_u_object::FLinearColor) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_SET_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_SET_COLOR,
                __buffer,
            )
        };
    }
    pub fn is_teleporting(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_IS_TELEPORTING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_IS_TELEPORTING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_aiming(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_IS_AIMING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_IS_AIMING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_vr_mode(&self) -> UPtr<UVREditorMode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_GET_VR_MODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_GET_VR_MODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UVREditorMode>>().read() }
    }
    pub fn get_interactor_trying_teleport(
        &self,
    ) -> UPtr<crate::bindings::viewport_interaction::UViewportInteractor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_GET_INTERACTOR_TRYING_TELEPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_GET_INTERACTOR_TRYING_TELEPORT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<crate::bindings::viewport_interaction::UViewportInteractor>,
                >()
                .read()
        }
    }
    pub fn do_teleport(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_DO_TELEPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::AVR_EDITOR_TELEPORTER_DO_TELEPORT,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UVREditorBaseUserWidget {
    __padding_end: [u8; 1296],
}
impl UVREditorBaseUserWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorBaseUserWidget")
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
#[repr(C, align(16))]
pub struct AVREditorBaseActor {
    __padding_end: [u8; 1536],
}
impl AVREditorBaseActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorBaseActor")
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
#[repr(C, align(16))]
pub struct AVREditorFloatingUI {
    __padding_end: [u8; 1808],
}
impl AVREditorFloatingUI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorFloatingUI")
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
#[repr(C, align(16))]
pub struct AVREditorDockableWindow {
    __padding_end: [u8; 1920],
}
impl AVREditorDockableWindow {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorDockableWindow")
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
#[repr(C, align(16))]
pub struct AVREditorDockableCameraWindow {
    __padding_end: [u8; 1920],
}
impl AVREditorDockableCameraWindow {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorDockableCameraWindow")
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
#[repr(C, align(16))]
pub struct UDockableWindowDragOperation {
    __padding_end: [u8; 288],
}
impl UDockableWindowDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDockableWindowDragOperation")
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
#[repr(C, align(16))]
pub struct AVREditorFloatingCameraUI {
    __padding_end: [u8; 1840],
}
impl AVREditorFloatingCameraUI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorFloatingCameraUI")
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
#[repr(C, align(16))]
pub struct AVREditorRadialFloatingUI {
    __padding_end: [u8; 1712],
}
impl AVREditorRadialFloatingUI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorRadialFloatingUI")
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
#[repr(C, align(16))]
pub struct UVREditorUISystem {
    __padding_end: [u8; 544],
}
impl UVREditorUISystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorUISystem")
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
pub struct UVRRadialMenuHandler {
    __padding_end: [u8; 216],
}
impl UVRRadialMenuHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRRadialMenuHandler")
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
pub struct AVREditorAvatarActor {
    __padding_end: [u8; 1248],
}
impl AVREditorAvatarActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorAvatarActor")
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
#[repr(C, align(16))]
pub struct UVREditorWidgetComponent {
    __padding_end: [u8; 2032],
}
impl UVREditorWidgetComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorWidgetComponent")
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
#[repr(C, align(16))]
pub struct UVREditorCameraWidgetComponent {
    __padding_end: [u8; 2032],
}
impl UVREditorCameraWidgetComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorCameraWidgetComponent")
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
pub struct AFloatingText {
    __padding_end: [u8; 1208],
}
impl AFloatingText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFloatingText")
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
#[repr(C, align(16))]
pub struct UVREditorInteractor {
    #[doc(hidden)]
    __padding_1968: [u8; 1968],
    pub hand_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    __padding_end: [u8; 600],
}
impl UVREditorInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorInteractor")
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
    pub fn try_override_controller_type(
        &mut self,
        in_controller_type: EControllerType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_TRY_OVERRIDE_CONTROLLER_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller_type,
                __buffer.add(0).cast::<EControllerType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_TRY_OVERRIDE_CONTROLLER_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_force_show_laser(&mut self, b_in_force_show: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_FORCE_SHOW_LASER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_force_show,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_FORCE_SHOW_LASER,
                __buffer,
            )
        };
    }
    pub fn set_force_laser_color(
        &mut self,
        in_color: &crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_FORCE_LASER_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_FORCE_LASER_COLOR,
                __buffer,
            )
        };
    }
    pub fn set_controller_type(&mut self, in_controller_type: EControllerType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_CONTROLLER_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller_type,
                __buffer.add(0).cast::<EControllerType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_CONTROLLER_TYPE,
                __buffer,
            )
        };
    }
    pub fn set_controller_hand_side(&mut self, in_controller_hand_side: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_CONTROLLER_HAND_SIDE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller_hand_side,
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
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_SET_CONTROLLER_HAND_SIDE,
                __buffer,
            )
        };
    }
    pub fn replace_hand_mesh_component(
        &mut self,
        new_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        mesh_scale: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_REPLACE_HAND_MESH_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_scale,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_REPLACE_HAND_MESH_COMPONENT,
                __buffer,
            )
        };
    }
    pub fn is_touching_trackpad(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_IS_TOUCHING_TRACKPAD,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_IS_TOUCHING_TRACKPAD,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_hovering_over_ui(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_IS_HOVERING_OVER_UI,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_IS_HOVERING_OVER_UI,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_clicking_on_ui(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_IS_CLICKING_ON_UI,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_IS_CLICKING_ON_UI,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_trackpad_position(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_TRACKPAD_POSITION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_TRACKPAD_POSITION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_teleport_actor(&mut self) -> UPtr<AVREditorTeleporter> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_TELEPORT_ACTOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_TELEPORT_ACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<AVREditorTeleporter>>().read() }
    }
    pub fn get_select_and_move_trigger_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_SELECT_AND_MOVE_TRIGGER_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_SELECT_AND_MOVE_TRIGGER_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_motion_controller_component(
        &self,
    ) -> UPtr<crate::bindings::head_mounted_display::UMotionControllerComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_MOTION_CONTROLLER_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_MOTION_CONTROLLER_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<
                        crate::bindings::head_mounted_display::UMotionControllerComponent,
                    >,
                >()
                .read()
        }
    }
    pub fn get_last_trackpad_position(
        &self,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_LAST_TRACKPAD_POSITION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_LAST_TRACKPAD_POSITION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_laser_start(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_LASER_START,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_LASER_START,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_laser_end(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_LASER_END,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_LASER_END,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_hmd_device_type(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_HMD_DEVICE_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_HMD_DEVICE_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_controller_type(&self) -> EControllerType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_CONTROLLER_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_CONTROLLER_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EControllerType>().read() }
    }
    pub fn get_controller_side(&self) -> crate::bindings::input_core::EControllerHand {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_CONTROLLER_SIDE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_CONTROLLER_SIDE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::input_core::EControllerHand>().read()
        }
    }
    pub fn get_controller_hand_side(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_CONTROLLER_HAND_SIDE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_INTERACTOR_GET_CONTROLLER_HAND_SIDE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct UVREditorModeBase {
    __padding_end: [u8; 152],
}
impl UVREditorModeBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorModeBase")
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
pub struct UVREditorMode {
    #[doc(hidden)]
    __padding_392: [u8; 392],
    pub interactor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub teleporter_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    __padding_end: [u8; 88],
}
impl UVREditorMode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorMode")
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
    pub fn set_game_view(&mut self, b_game_view: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_MODE_SET_GAME_VIEW,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_MODE_SET_GAME_VIEW,
                __buffer,
            )
        };
    }
    pub fn is_in_game_view(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_MODE_IS_IN_GAME_VIEW,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_MODE_IS_IN_GAME_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_world_scale_factor(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_EDITOR_MODE_GET_WORLD_SCALE_FACTOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_EDITOR_MODE_GET_WORLD_SCALE_FACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UVREditorPlacement {
    __padding_end: [u8; 80],
}
impl UVREditorPlacement {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorPlacement")
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
pub struct UVRModeSettings {
    __padding_end: [u8; 136],
}
impl UVRModeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRModeSettings")
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
#[repr(C, align(16))]
pub struct UVRScoutingInteractor {
    #[doc(hidden)]
    __padding_2576: [u8; 2576],
    pub flying_indicator_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    #[doc(hidden)]
    __padding_2592: [u8; 8],
    pub b_receives_editor_input: bool,
}
impl UVRScoutingInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRScoutingInteractor")
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
    pub fn set_receives_editor_input(&mut self, b_in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_SET_RECEIVES_EDITOR_INPUT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_value,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_SET_RECEIVES_EDITOR_INPUT,
                __buffer,
            )
        };
    }
    pub fn set_gizmo_mode(
        &mut self,
        in_gizmo_mode: crate::bindings::viewport_interaction::EGizmoHandleTypes,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_SET_GIZMO_MODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_gizmo_mode,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::viewport_interaction::EGizmoHandleTypes>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_SET_GIZMO_MODE,
                __buffer,
            )
        };
    }
    pub fn get_selected_actors() -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_SELECTED_ACTORS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::vr_editor::UVRScoutingInteractor::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_SELECTED_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_receives_editor_input(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_RECEIVES_EDITOR_INPUT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_RECEIVES_EDITOR_INPUT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_input_component(&self) -> UPtr<crate::bindings::engine::UInputComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_INPUT_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_INPUT_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UInputComponent>>()
                .read()
        }
    }
    pub fn get_gizmo_mode(
        &self,
    ) -> crate::bindings::viewport_interaction::EGizmoHandleTypes {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_GIZMO_MODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::UVR_SCOUTING_INTERACTOR_GET_GIZMO_MODE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::viewport_interaction::EGizmoHandleTypes>()
                .read()
        }
    }
}
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
