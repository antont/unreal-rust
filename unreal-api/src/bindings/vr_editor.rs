#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub avr_editor_teleporter_teleport_done: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_stop_aiming: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_start_teleport: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_start_aiming: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_shutdown: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_set_visibility: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_set_color: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_is_teleporting: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_is_aiming: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_init: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_get_vr_mode: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_get_slide_delta: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_get_interactor_trying_teleport: *mut crate::ffi::UFunctionOpague,
    pub avr_editor_teleporter_do_teleport: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_update_hand_mesh_relative_transform: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_try_override_controller_type: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_setup_component: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_set_force_show_laser: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_set_force_laser_color: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_set_controller_type: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_set_controller_hand_side: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_replace_hand_mesh_component: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_is_touching_trackpad: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_is_hovering_over_ui: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_is_clicking_on_ui: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_init: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_trackpad_position: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_teleport_actor: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_slide_delta: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_select_and_move_trigger_value: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_motion_controller_component: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_last_trackpad_position: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_laser_start: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_laser_end: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_hmd_device_type: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_controller_type: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_controller_side: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_interactor_get_controller_hand_side: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_mode_set_game_view: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_mode_is_in_game_view: *mut crate::ffi::UFunctionOpague,
    pub uvr_editor_mode_get_world_scale_factor: *mut crate::ffi::UFunctionOpague,
    pub uvr_scouting_interactor_set_receives_editor_input: *mut crate::ffi::UFunctionOpague,
    pub uvr_scouting_interactor_set_gizmo_mode: *mut crate::ffi::UFunctionOpague,
    pub uvr_scouting_interactor_get_selected_actors: *mut crate::ffi::UFunctionOpague,
    pub uvr_scouting_interactor_get_receives_editor_input: *mut crate::ffi::UFunctionOpague,
    pub uvr_scouting_interactor_get_input_component: *mut crate::ffi::UFunctionOpague,
    pub uvr_scouting_interactor_get_gizmo_mode: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            avr_editor_teleporter_teleport_done: std::ptr::null_mut(),
            avr_editor_teleporter_stop_aiming: std::ptr::null_mut(),
            avr_editor_teleporter_start_teleport: std::ptr::null_mut(),
            avr_editor_teleporter_start_aiming: std::ptr::null_mut(),
            avr_editor_teleporter_shutdown: std::ptr::null_mut(),
            avr_editor_teleporter_set_visibility: std::ptr::null_mut(),
            avr_editor_teleporter_set_color: std::ptr::null_mut(),
            avr_editor_teleporter_is_teleporting: std::ptr::null_mut(),
            avr_editor_teleporter_is_aiming: std::ptr::null_mut(),
            avr_editor_teleporter_init: std::ptr::null_mut(),
            avr_editor_teleporter_get_vr_mode: std::ptr::null_mut(),
            avr_editor_teleporter_get_slide_delta: std::ptr::null_mut(),
            avr_editor_teleporter_get_interactor_trying_teleport: std::ptr::null_mut(),
            avr_editor_teleporter_do_teleport: std::ptr::null_mut(),
            uvr_editor_interactor_update_hand_mesh_relative_transform: std::ptr::null_mut(),
            uvr_editor_interactor_try_override_controller_type: std::ptr::null_mut(),
            uvr_editor_interactor_setup_component: std::ptr::null_mut(),
            uvr_editor_interactor_set_force_show_laser: std::ptr::null_mut(),
            uvr_editor_interactor_set_force_laser_color: std::ptr::null_mut(),
            uvr_editor_interactor_set_controller_type: std::ptr::null_mut(),
            uvr_editor_interactor_set_controller_hand_side: std::ptr::null_mut(),
            uvr_editor_interactor_replace_hand_mesh_component: std::ptr::null_mut(),
            uvr_editor_interactor_is_touching_trackpad: std::ptr::null_mut(),
            uvr_editor_interactor_is_hovering_over_ui: std::ptr::null_mut(),
            uvr_editor_interactor_is_clicking_on_ui: std::ptr::null_mut(),
            uvr_editor_interactor_init: std::ptr::null_mut(),
            uvr_editor_interactor_get_trackpad_position: std::ptr::null_mut(),
            uvr_editor_interactor_get_teleport_actor: std::ptr::null_mut(),
            uvr_editor_interactor_get_slide_delta: std::ptr::null_mut(),
            uvr_editor_interactor_get_select_and_move_trigger_value: std::ptr::null_mut(),
            uvr_editor_interactor_get_motion_controller_component: std::ptr::null_mut(),
            uvr_editor_interactor_get_last_trackpad_position: std::ptr::null_mut(),
            uvr_editor_interactor_get_laser_start: std::ptr::null_mut(),
            uvr_editor_interactor_get_laser_end: std::ptr::null_mut(),
            uvr_editor_interactor_get_hmd_device_type: std::ptr::null_mut(),
            uvr_editor_interactor_get_controller_type: std::ptr::null_mut(),
            uvr_editor_interactor_get_controller_side: std::ptr::null_mut(),
            uvr_editor_interactor_get_controller_hand_side: std::ptr::null_mut(),
            uvr_editor_mode_set_game_view: std::ptr::null_mut(),
            uvr_editor_mode_is_in_game_view: std::ptr::null_mut(),
            uvr_editor_mode_get_world_scale_factor: std::ptr::null_mut(),
            uvr_scouting_interactor_set_receives_editor_input: std::ptr::null_mut(),
            uvr_scouting_interactor_set_gizmo_mode: std::ptr::null_mut(),
            uvr_scouting_interactor_get_selected_actors: std::ptr::null_mut(),
            uvr_scouting_interactor_get_receives_editor_input: std::ptr::null_mut(),
            uvr_scouting_interactor_get_input_component: std::ptr::null_mut(),
            uvr_scouting_interactor_get_gizmo_mode: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AVREditorTeleporter::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TeleportDone"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_teleport_done,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopAiming"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_stop_aiming,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartTeleport"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_start_teleport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartAiming"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_start_aiming,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Shutdown"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_shutdown,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVisibility"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_set_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetColor"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_set_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTeleporting"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_is_teleporting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAiming"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_is_aiming,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Init"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_init,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVRMode"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_get_vr_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlideDelta"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_get_slide_delta,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInteractorTryingTeleport"),
                &raw mut __FUNCTION_PTRS
                    .avr_editor_teleporter_get_interactor_trying_teleport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoTeleport"),
                &raw mut __FUNCTION_PTRS.avr_editor_teleporter_do_teleport,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UVREditorInteractor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateHandMeshRelativeTransform"),
                &raw mut __FUNCTION_PTRS
                    .uvr_editor_interactor_update_hand_mesh_relative_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryOverrideControllerType"),
                &raw mut __FUNCTION_PTRS
                    .uvr_editor_interactor_try_override_controller_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetupComponent"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_setup_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetForceShowLaser"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_set_force_show_laser,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetForceLaserColor"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_set_force_laser_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControllerType"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_set_controller_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControllerHandSide"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_set_controller_hand_side,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceHandMeshComponent"),
                &raw mut __FUNCTION_PTRS
                    .uvr_editor_interactor_replace_hand_mesh_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTouchingTrackpad"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_is_touching_trackpad,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHoveringOverUI"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_is_hovering_over_ui,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsClickingOnUI"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_is_clicking_on_ui,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Init"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_init,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrackpadPosition"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_trackpad_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTeleportActor"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_teleport_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlideDelta"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_slide_delta,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectAndMoveTriggerValue"),
                &raw mut __FUNCTION_PTRS
                    .uvr_editor_interactor_get_select_and_move_trigger_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMotionControllerComponent"),
                &raw mut __FUNCTION_PTRS
                    .uvr_editor_interactor_get_motion_controller_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastTrackpadPosition"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_last_trackpad_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLaserStart"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_laser_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLaserEnd"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_laser_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHMDDeviceType"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_hmd_device_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControllerType"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_controller_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControllerSide"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_controller_side,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControllerHandSide"),
                &raw mut __FUNCTION_PTRS.uvr_editor_interactor_get_controller_hand_side,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UVREditorMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGameView"),
                &raw mut __FUNCTION_PTRS.uvr_editor_mode_set_game_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInGameView"),
                &raw mut __FUNCTION_PTRS.uvr_editor_mode_is_in_game_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWorldScaleFactor"),
                &raw mut __FUNCTION_PTRS.uvr_editor_mode_get_world_scale_factor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UVRScoutingInteractor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetReceivesEditorInput"),
                &raw mut __FUNCTION_PTRS
                    .uvr_scouting_interactor_set_receives_editor_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGizmoMode"),
                &raw mut __FUNCTION_PTRS.uvr_scouting_interactor_set_gizmo_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedActors"),
                &raw mut __FUNCTION_PTRS.uvr_scouting_interactor_get_selected_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReceivesEditorInput"),
                &raw mut __FUNCTION_PTRS
                    .uvr_scouting_interactor_get_receives_editor_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInputComponent"),
                &raw mut __FUNCTION_PTRS.uvr_scouting_interactor_get_input_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGizmoMode"),
                &raw mut __FUNCTION_PTRS.uvr_scouting_interactor_get_gizmo_mode,
            );
        }
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorAssetContainer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorAssetContainer")
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
pub struct UVREditorAutoScaler {
    __padding_end: [u8; 56],
}
impl UVREditorAutoScaler {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorAutoScaler")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorAutoScaler")
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
pub struct AVREditorTeleporter {
    __padding_end: [u8; 1312],
}
impl AVREditorTeleporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorTeleporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorTeleporter")
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
    pub fn teleport_done(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_teleport_done,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_teleport_done,
                __buffer,
            )
        };
    }
    pub fn stop_aiming(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_stop_aiming,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_stop_aiming,
                __buffer,
            )
        };
    }
    pub fn start_teleport(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_start_teleport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_start_teleport,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_start_aiming,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_start_aiming,
                __buffer,
            )
        };
    }
    pub fn shutdown(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_shutdown,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_shutdown,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_set_visibility,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_set_visibility,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_set_color,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_set_color,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_is_teleporting,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_is_teleporting,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_is_aiming,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_is_aiming,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn init(&mut self, in_mode: UPtr<UVREditorMode>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS.avr_editor_teleporter_init,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mode,
                __buffer.add(0).cast::<UPtr<UVREditorMode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS.avr_editor_teleporter_init,
                __buffer,
            )
        };
    }
    pub fn get_vr_mode(&self) -> UPtr<UVREditorMode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_get_vr_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_get_vr_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UVREditorMode>>().read() }
    }
    pub fn get_slide_delta(
        &mut self,
        interactor: UPtr<UVREditorInteractor>,
        axis: bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_get_slide_delta,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interactor,
                __buffer.add(0).cast::<UPtr<UVREditorInteractor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&axis, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_get_slide_delta,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_get_interactor_trying_teleport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_get_interactor_trying_teleport,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_do_teleport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .avr_editor_teleporter_do_teleport,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorBaseUserWidget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorBaseUserWidget")
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
#[repr(C, align(16))]
pub struct AVREditorBaseActor {
    __padding_end: [u8; 1536],
}
impl AVREditorBaseActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorBaseActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorBaseActor")
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
#[repr(C, align(16))]
pub struct AVREditorFloatingUI {
    __padding_end: [u8; 1808],
}
impl AVREditorFloatingUI {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorFloatingUI")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorFloatingUI")
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
#[repr(C, align(16))]
pub struct AVREditorDockableWindow {
    __padding_end: [u8; 1920],
}
impl AVREditorDockableWindow {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorDockableWindow")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorDockableWindow")
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
#[repr(C, align(16))]
pub struct AVREditorDockableCameraWindow {
    __padding_end: [u8; 1920],
}
impl AVREditorDockableCameraWindow {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorDockableCameraWindow")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorDockableCameraWindow")
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
#[repr(C, align(16))]
pub struct UDockableWindowDragOperation {
    __padding_end: [u8; 288],
}
impl UDockableWindowDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDockableWindowDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDockableWindowDragOperation")
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
#[repr(C, align(16))]
pub struct AVREditorFloatingCameraUI {
    __padding_end: [u8; 1840],
}
impl AVREditorFloatingCameraUI {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorFloatingCameraUI")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorFloatingCameraUI")
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
#[repr(C, align(16))]
pub struct AVREditorRadialFloatingUI {
    __padding_end: [u8; 1712],
}
impl AVREditorRadialFloatingUI {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorRadialFloatingUI")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorRadialFloatingUI")
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
#[repr(C, align(16))]
pub struct UVREditorUISystem {
    __padding_end: [u8; 544],
}
impl UVREditorUISystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorUISystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorUISystem")
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
pub struct UVRRadialMenuHandler {
    __padding_end: [u8; 216],
}
impl UVRRadialMenuHandler {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRRadialMenuHandler")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRRadialMenuHandler")
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
pub struct AVREditorAvatarActor {
    __padding_end: [u8; 1248],
}
impl AVREditorAvatarActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorAvatarActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVREditorAvatarActor")
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
#[repr(C, align(16))]
pub struct UVREditorWidgetComponent {
    __padding_end: [u8; 2032],
}
impl UVREditorWidgetComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorWidgetComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorWidgetComponent")
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
#[repr(C, align(16))]
pub struct UVREditorCameraWidgetComponent {
    __padding_end: [u8; 2032],
}
impl UVREditorCameraWidgetComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorCameraWidgetComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorCameraWidgetComponent")
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
pub struct AFloatingText {
    __padding_end: [u8; 1208],
}
impl AFloatingText {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFloatingText")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFloatingText")
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
#[repr(C, align(16))]
pub struct UVREditorInteractor {
    #[doc(hidden)]
    pub(crate) __padding_1968: [u8; 1968],
    pub hand_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    __padding_end: [u8; 600],
}
impl UVREditorInteractor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorInteractor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorInteractor")
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
    pub fn update_hand_mesh_relative_transform(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_update_hand_mesh_relative_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_update_hand_mesh_relative_transform,
                __buffer,
            )
        };
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_try_override_controller_type,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_try_override_controller_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn setup_component(
        &mut self,
        owning_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_setup_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_setup_component,
                __buffer,
            )
        };
    }
    pub fn set_force_show_laser(&mut self, b_in_force_show: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_force_show_laser,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_force_show_laser,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_force_laser_color,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_force_laser_color,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_controller_type,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_controller_type,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_controller_hand_side,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_set_controller_hand_side,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_replace_hand_mesh_component,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_replace_hand_mesh_component,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_is_touching_trackpad,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_is_touching_trackpad,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_is_hovering_over_ui,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_is_hovering_over_ui,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_is_clicking_on_ui,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_is_clicking_on_ui,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn init(&mut self, in_vr_mode: UPtr<UVREditorMode>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS.uvr_editor_interactor_init,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_vr_mode,
                __buffer.add(0).cast::<UPtr<UVREditorMode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS.uvr_editor_interactor_init,
                __buffer,
            )
        };
    }
    pub fn get_trackpad_position(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_trackpad_position,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_trackpad_position,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_teleport_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_teleport_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<AVREditorTeleporter>>().read() }
    }
    pub fn get_slide_delta(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_slide_delta,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_slide_delta,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_select_and_move_trigger_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_select_and_move_trigger_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_select_and_move_trigger_value,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_motion_controller_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_motion_controller_component,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_last_trackpad_position,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_last_trackpad_position,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_laser_start,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_laser_start,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_laser_end,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_laser_end,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_hmd_device_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_hmd_device_type,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_controller_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_controller_type,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_controller_side,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_controller_side,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_controller_hand_side,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_interactor_get_controller_hand_side,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorModeBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorModeBase")
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
pub struct UVREditorMode {
    #[doc(hidden)]
    pub(crate) __padding_392: [u8; 392],
    pub interactor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub teleporter_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    __padding_end: [u8; 88],
}
impl UVREditorMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorMode")
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
    pub fn set_game_view(&mut self, b_game_view: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_mode_set_game_view,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_mode_set_game_view,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_mode_is_in_game_view,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_mode_is_in_game_view,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_mode_get_world_scale_factor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_editor_mode_get_world_scale_factor,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorPlacement")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVREditorPlacement")
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
pub struct UVRModeSettings {
    __padding_end: [u8; 136],
}
impl UVRModeSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRModeSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRModeSettings")
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
#[repr(C, align(16))]
pub struct UVRScoutingInteractor {
    #[doc(hidden)]
    pub(crate) __padding_2576: [u8; 2576],
    pub flying_indicator_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    #[doc(hidden)]
    pub(crate) __padding_2592: [u8; 8],
    pub b_receives_editor_input: bool,
}
impl UVRScoutingInteractor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRScoutingInteractor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVRScoutingInteractor")
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
    pub fn set_receives_editor_input(&mut self, b_in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_set_receives_editor_input,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_set_receives_editor_input,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_set_gizmo_mode,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_set_gizmo_mode,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_selected_actors,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::vr_editor::UVRScoutingInteractor::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_selected_actors,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_receives_editor_input,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_receives_editor_input,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_input_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_input_component,
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
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_gizmo_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::vr_editor::__FUNCTION_PTRS
                    .uvr_scouting_interactor_get_gizmo_mode,
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
