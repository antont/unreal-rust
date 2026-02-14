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
    pub u_chaos_mover_backend_component_handle_updated_component_physics_state_changed: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_backend_component_handle_owning_pawn_controller_changed_server: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_movement_mode_get_simulation: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_movement_mode_transition_get_simulation_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_movement_mode_transition_get_simulation: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_k2_queue_movement_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_k2_queue_instant_movement_effect: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_has_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_get_local_sim_input_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_get_local_sim_input: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_get_debug_sim_data: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_get_current_movement_mode: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_find_movement_mode_by_name_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_find_movement_mode_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_mover_simulation_cancel_modifier_from_handle: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_should_character_remain_upright: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_override_max_speed: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_override_acceleration: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_get_target_height: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_get_max_walk_slope_cosine: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_get_max_speed: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_get_ground_query_radius: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_get_acceleration: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_clear_max_speed_override: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_movement_mode_interface_clear_acceleration_override: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_mover_component_override_movement_settings: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_mover_component_launch: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_character_mover_component_cancel_movement_settings_overrides: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_mode_set_one_way_trip_duration_begin_play_only: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_mode_bp_find_pattern: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_wants_reverse_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_wants_playing_path: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_wants_one_way_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_wants_looping_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_request_stop_playing_path: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_request_start_playing_path: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_request_reverse_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_request_one_way_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_request_looping_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_on_post_sim_event_received: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_on_post_finalize: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_is_reverse_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_is_playing_path: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_is_one_way_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_controller_component_is_looping_playback: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_debug_draw_interface_should_display_progress_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_debug_draw_interface_get_progress_preview_mesh_material: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_pathed_movement_debug_draw_interface_get_preview_mesh_overall_path_progress: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_ground_movement_utils_compute_local_ground_velocity_internal: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_chaos_mover_backend_component_handle_updated_component_physics_state_changed: std::ptr::null_mut(),
            u_chaos_mover_backend_component_handle_owning_pawn_controller_changed_server: std::ptr::null_mut(),
            u_chaos_movement_mode_get_simulation: std::ptr::null_mut(),
            u_chaos_movement_mode_transition_get_simulation_mutable: std::ptr::null_mut(),
            u_chaos_movement_mode_transition_get_simulation: std::ptr::null_mut(),
            u_chaos_mover_simulation_k2_queue_movement_modifier: std::ptr::null_mut(),
            u_chaos_mover_simulation_k2_queue_instant_movement_effect: std::ptr::null_mut(),
            u_chaos_mover_simulation_has_gameplay_tag: std::ptr::null_mut(),
            u_chaos_mover_simulation_get_local_sim_input_mutable: std::ptr::null_mut(),
            u_chaos_mover_simulation_get_local_sim_input: std::ptr::null_mut(),
            u_chaos_mover_simulation_get_debug_sim_data: std::ptr::null_mut(),
            u_chaos_mover_simulation_get_current_movement_mode: std::ptr::null_mut(),
            u_chaos_mover_simulation_find_movement_mode_by_name_mutable: std::ptr::null_mut(),
            u_chaos_mover_simulation_find_movement_mode_by_name: std::ptr::null_mut(),
            u_chaos_mover_simulation_cancel_modifier_from_handle: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_should_character_remain_upright: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_override_max_speed: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_override_acceleration: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_get_target_height: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_get_max_walk_slope_cosine: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_get_max_speed: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_get_ground_query_radius: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_get_acceleration: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_clear_max_speed_override: std::ptr::null_mut(),
            u_chaos_character_movement_mode_interface_clear_acceleration_override: std::ptr::null_mut(),
            u_chaos_character_mover_component_override_movement_settings: std::ptr::null_mut(),
            u_chaos_character_mover_component_launch: std::ptr::null_mut(),
            u_chaos_character_mover_component_cancel_movement_settings_overrides: std::ptr::null_mut(),
            u_chaos_pathed_movement_mode_set_one_way_trip_duration_begin_play_only: std::ptr::null_mut(),
            u_chaos_pathed_movement_mode_bp_find_pattern: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_wants_reverse_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_wants_playing_path: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_wants_one_way_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_wants_looping_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_request_stop_playing_path: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_request_start_playing_path: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_request_reverse_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_request_one_way_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_request_looping_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_on_post_sim_event_received: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_on_post_finalize: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_is_reverse_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_is_playing_path: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_is_one_way_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_controller_component_is_looping_playback: std::ptr::null_mut(),
            u_chaos_pathed_movement_debug_draw_interface_should_display_progress_preview_mesh: std::ptr::null_mut(),
            u_chaos_pathed_movement_debug_draw_interface_get_progress_preview_mesh_material: std::ptr::null_mut(),
            u_chaos_pathed_movement_debug_draw_interface_get_preview_mesh_overall_path_progress: std::ptr::null_mut(),
            u_chaos_ground_movement_utils_compute_local_ground_velocity_internal: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosMoverBackendComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleUpdatedComponentPhysicsStateChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_backend_component_handle_updated_component_physics_state_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleOwningPawnControllerChanged_Server"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_backend_component_handle_owning_pawn_controller_changed_server,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosMovementMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSimulation"),
                &raw mut __FUNCTION_PTRS.u_chaos_movement_mode_get_simulation,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosMovementModeTransition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSimulation_Mutable"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_movement_mode_transition_get_simulation_mutable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSimulation"),
                &raw mut __FUNCTION_PTRS.u_chaos_movement_mode_transition_get_simulation,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosMoverSimulation::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_QueueMovementModifier"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_k2_queue_movement_modifier,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_QueueInstantMovementEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_k2_queue_instant_movement_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasGameplayTag"),
                &raw mut __FUNCTION_PTRS.u_chaos_mover_simulation_has_gameplay_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalSimInput_Mutable"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_local_sim_input_mutable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalSimInput"),
                &raw mut __FUNCTION_PTRS.u_chaos_mover_simulation_get_local_sim_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDebugSimData"),
                &raw mut __FUNCTION_PTRS.u_chaos_mover_simulation_get_debug_sim_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentMovementMode"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_current_movement_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindMovementModeByName_Mutable"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_find_movement_mode_by_name_mutable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindMovementModeByName"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_find_movement_mode_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelModifierFromHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_mover_simulation_cancel_modifier_from_handle,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosCharacterMovementModeInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldCharacterRemainUpright"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_should_character_remain_upright,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideMaxSpeed"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_override_max_speed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideAcceleration"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_override_acceleration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetHeight"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_target_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMaxWalkSlopeCosine"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_max_walk_slope_cosine,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMaxSpeed"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_max_speed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGroundQueryRadius"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_ground_query_radius,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAcceleration"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_acceleration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearMaxSpeedOverride"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_clear_max_speed_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearAccelerationOverride"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_clear_acceleration_override,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosCharacterMoverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideMovementSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_mover_component_override_movement_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Launch"),
                &raw mut __FUNCTION_PTRS.u_chaos_character_mover_component_launch,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelMovementSettingsOverrides"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_character_mover_component_cancel_movement_settings_overrides,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosPathedMovementMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOneWayTripDuration_BeginPlayOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_mode_set_one_way_trip_duration_begin_play_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_FindPattern"),
                &raw mut __FUNCTION_PTRS.u_chaos_pathed_movement_mode_bp_find_pattern,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosPathedMovementControllerComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WantsReversePlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_reverse_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WantsPlayingPath"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_playing_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WantsOneWayPlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_one_way_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WantsLoopingPlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_looping_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestStopPlayingPath"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_stop_playing_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestStartPlayingPath"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_start_playing_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestReversePlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_reverse_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestOneWayPlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_one_way_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestLoopingPlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_looping_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPostSimEventReceived"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_on_post_sim_event_received,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPostFinalize"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_on_post_finalize,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReversePlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_reverse_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPlayingPath"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_playing_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsOneWayPlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_one_way_playback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLoopingPlayback"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_looping_playback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosPathedMovementDebugDrawInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldDisplayProgressPreviewMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_should_display_progress_preview_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetProgressPreviewMeshMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_get_progress_preview_mesh_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviewMeshOverallPathProgress"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_get_preview_mesh_overall_path_progress,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChaosGroundMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeLocalGroundVelocity_Internal"),
                &raw mut __FUNCTION_PTRS
                    .u_chaos_ground_movement_utils_compute_local_ground_velocity_internal,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FChaosMoverCharacterSimState {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target_delta_position: crate::bindings::core_u_object::FVector,
    pub target_delta_facing: f32,
}
impl FChaosMoverCharacterSimState {}
#[repr(C, align(8))]
pub struct FChaosMoverGroundSimState {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub local_velocity: crate::bindings::core_u_object::FVector,
}
impl FChaosMoverGroundSimState {}
#[repr(C, align(16))]
pub struct FChaosMovementBasis {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub basis_location: crate::bindings::core_u_object::FVector,
    pub basis_rotation: crate::bindings::core_u_object::FQuat,
}
impl FChaosMovementBasis {}
#[repr(C, align(8))]
pub struct FChaosMoverSimulationDefaultInputs {
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 184],
    pub up_dir: crate::bindings::core_u_object::FVector,
    pub gravity: crate::bindings::core_u_object::FVector,
    pub b_is_generating_inputs_locally: bool,
    pub b_is_remotely_controlled: bool,
    #[doc(hidden)]
    pub(crate) __padding_264: [u8; 28],
    pub physics_object_gravity: f32,
    pub pawn_collision_half_height: f32,
    pub pawn_collision_radius: f32,
}
impl FChaosMoverSimulationDefaultInputs {}
#[repr(C, align(8))]
pub struct FChaosMoverTimeStepDebugData {
    pub(crate) __padding_end: [u8; 40],
}
impl FChaosMoverTimeStepDebugData {}
#[repr(C, align(8))]
pub struct FChaosNetInstantMovementEffectsQueue {
    pub(crate) __padding_end: [u8; 24],
}
impl FChaosNetInstantMovementEffectsQueue {}
#[repr(C, align(8))]
pub struct FChaosMoverLaunchInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub launch_velocity_or_impulse: crate::bindings::core_u_object::FVector,
    pub mode: EChaosMoverVelocityEffectMode,
}
impl FChaosMoverLaunchInputs {}
#[repr(C, align(8))]
pub struct FChaosMoverCrouchInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub b_wants_to_crouch: bool,
}
impl FChaosMoverCrouchInputs {}
#[repr(C, align(8))]
pub struct FChaosMovementSettingsOverrides {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub mode_name: FName,
    pub max_speed_override: f32,
    pub acceleration_override: f32,
}
impl FChaosMovementSettingsOverrides {}
#[repr(C, align(8))]
pub struct FChaosMovementSettingsOverridesRemover {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub mode_name: FName,
}
impl FChaosMovementSettingsOverridesRemover {}
#[repr(C, align(8))]
pub struct FChaosCharacterApplyVelocityEffect {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub velocity_or_impulse_to_apply: crate::bindings::core_u_object::FVector,
    pub mode: EChaosMoverVelocityEffectMode,
}
impl FChaosCharacterApplyVelocityEffect {}
#[repr(C, align(4))]
pub struct FSwimmingSettings {
    pub max_speed_up: f32,
    pub max_speed_down: f32,
    pub water_velocity_depth_for_max: f32,
    pub water_velocity_min_multiplier: f32,
    pub max_water_force: f32,
    pub water_force_multiplier: f32,
    pub water_force_second_multiplier: f32,
    pub bobbing_max_force: f32,
    pub bobbing_ideal_depth_tolerance: f32,
    pub bobbing_friction_down: f32,
    pub bobbing_exp_drag_down: f32,
    pub bobbing_friction_down_submerged: f32,
    pub bobbing_exp_drag_down_submerged: f32,
    pub bobbing_friction_up: f32,
    pub bobbing_exp_drag_up: f32,
    pub bobbing_friction_multiplier: f32,
    pub bobbing_exp_drag_multiplier: f32,
}
impl FSwimmingSettings {}
#[repr(C, align(8))]
pub struct FChaosStanceModifier {
    #[doc(hidden)]
    pub(crate) __padding_33: [u8; 33],
    pub b_cancel_on_mode_change: bool,
    pub modified_capsule_half_height: f32,
    pub modified_capsule_radius: f32,
    pub modified_capsule_ground_clearance: f32,
    pub default_capsule_half_height: f32,
    pub default_capsule_radius: f32,
    pub default_capsule_ground_clearance: f32,
    pub max_speed_override: TOptional<f32>,
    pub acceleration_override: TOptional<f32>,
    pub(crate) __padding_end: [u8; 12],
}
impl FChaosStanceModifier {}
#[repr(C, align(8))]
pub struct FChaosPathedMovementInputs {
    pub(crate) __padding_end: [u8; 24],
}
impl FChaosPathedMovementInputs {}
#[repr(C, align(8))]
pub struct FChaosPathedMovementState {
    pub(crate) __padding_end: [u8; 32],
}
impl FChaosPathedMovementState {}
#[repr(C, align(8))]
pub struct FChaosPathedMovementModeDebugData {
    pub(crate) __padding_end: [u8; 184],
}
impl FChaosPathedMovementModeDebugData {}
#[repr(C, align(16))]
pub struct FChaosPointMovementPathPoint {
    pub basis: EChaosPointMovementLocationBasis,
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation_basis: EChaosPointMovementLocationBasis,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub(crate) __padding_end: [u8; 80],
}
impl FChaosPointMovementPathPoint {}
#[repr(C, align(8))]
pub struct USharedChaosCharacterMovementSettings {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub deceleration: f32,
    pub acceleration: f32,
    pub turning_rate: f32,
    pub turning_boost: f32,
    pub max_speed: f32,
    pub b_use_acceleration_for_velocity_move: bool,
    pub ground_friction: f32,
    pub flags_84: u8,
    pub braking_friction: f32,
    pub braking_friction_factor: f32,
    pub max_step_height: f32,
    pub default_falling_mode: FName,
    pub max_walkable_slope_angle: f32,
}
impl USharedChaosCharacterMovementSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USharedChaosCharacterMovementSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USharedChaosCharacterMovementSettings")
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
pub struct UChaosMoverBackendComponent {
    __padding_end: [u8; 2000],
}
impl UChaosMoverBackendComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMoverBackendComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMoverBackendComponent")
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
pub struct UChaosMoverSubsystem {
    __padding_end: [u8; 104],
}
impl UChaosMoverSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMoverSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMoverSubsystem")
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
pub struct UChaosMovementMode {
    __padding_end: [u8; 136],
}
impl UChaosMovementMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMovementMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMovementMode")
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
    pub fn get_simulation(&self) -> UPtr<UChaosMoverSimulation> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_movement_mode_get_simulation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_movement_mode_get_simulation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UChaosMoverSimulation>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UChaosMovementModeTransition {
    __padding_end: [u8; 80],
}
impl UChaosMovementModeTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMovementModeTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMovementModeTransition")
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
    pub fn get_simulation_mutable(&mut self) -> UPtr<UChaosMoverSimulation> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_movement_mode_transition_get_simulation_mutable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_movement_mode_transition_get_simulation_mutable,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UChaosMoverSimulation>>().read() }
    }
    pub fn get_simulation(&self) -> UPtr<UChaosMoverSimulation> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_movement_mode_transition_get_simulation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_movement_mode_transition_get_simulation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UChaosMoverSimulation>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UChaosMoverSimulation {
    __padding_end: [u8; 1776],
}
impl UChaosMoverSimulation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMoverSimulation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMoverSimulation")
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
    pub fn k2_queue_movement_modifier(
        &mut self,
        move_as_raw_data: &i32,
    ) -> crate::bindings::mover::FMovementModifierHandle {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_k2_queue_movement_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_as_raw_data,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_k2_queue_movement_modifier,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(4)
                .cast::<crate::bindings::mover::FMovementModifierHandle>()
                .read()
        }
    }
    pub fn k2_queue_instant_movement_effect(&mut self, effect_as_raw_data: &i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_k2_queue_instant_movement_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_as_raw_data,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_k2_queue_instant_movement_effect,
                __buffer,
            )
        };
    }
    pub fn has_gameplay_tag(
        &self,
        tag_to_find: crate::bindings::gameplay_tags::FGameplayTag,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_has_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_find,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_has_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn get_local_sim_input_mutable(
        &mut self,
    ) -> crate::bindings::mover::FMoverDataCollection {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_local_sim_input_mutable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_local_sim_input_mutable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::mover::FMoverDataCollection>().read()
        }
    }
    pub fn get_local_sim_input(&self) -> crate::bindings::mover::FMoverDataCollection {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_local_sim_input,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_local_sim_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::mover::FMoverDataCollection>().read()
        }
    }
    pub fn get_debug_sim_data(
        &mut self,
    ) -> crate::bindings::mover::FMoverDataCollection {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_debug_sim_data,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_debug_sim_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::mover::FMoverDataCollection>().read()
        }
    }
    pub fn get_current_movement_mode(
        &self,
    ) -> UPtr<crate::bindings::mover::UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_current_movement_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_get_current_movement_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::mover::UBaseMovementMode>>()
                .read()
        }
    }
    pub fn find_movement_mode_by_name_mutable(
        &mut self,
        name: &FName,
    ) -> UPtr<crate::bindings::mover::UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_find_movement_mode_by_name_mutable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_find_movement_mode_by_name_mutable,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::mover::UBaseMovementMode>>()
                .read()
        }
    }
    pub fn find_movement_mode_by_name(
        &self,
        name: &FName,
    ) -> UPtr<crate::bindings::mover::UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_find_movement_mode_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_find_movement_mode_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::mover::UBaseMovementMode>>()
                .read()
        }
    }
    pub fn cancel_modifier_from_handle(
        &mut self,
        modifier_handle: crate::bindings::mover::FMovementModifierHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_cancel_modifier_from_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modifier_handle,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::mover::FMovementModifierHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_mover_simulation_cancel_modifier_from_handle,
                __buffer,
            )
        };
    }
}
pub struct IChaosCharacterMovementModeInterface {}
#[repr(C, align(8))]
pub struct UChaosCharacterMovementModeInterface {
    __padding_end: [u8; 48],
}
impl UChaosCharacterMovementModeInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterMovementModeInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterMovementModeInterface")
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
    pub fn should_character_remain_upright(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_should_character_remain_upright,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_should_character_remain_upright,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn override_max_speed(&mut self, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_override_max_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_override_max_speed,
                __buffer,
            )
        };
    }
    pub fn override_acceleration(&mut self, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_override_acceleration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_override_acceleration,
                __buffer,
            )
        };
    }
    pub fn get_target_height(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_target_height,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_target_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_max_walk_slope_cosine(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_max_walk_slope_cosine,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_max_walk_slope_cosine,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_max_speed(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_max_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_max_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_ground_query_radius(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_ground_query_radius,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_ground_query_radius,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_acceleration(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_acceleration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_get_acceleration,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn clear_max_speed_override(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_clear_max_speed_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_clear_max_speed_override,
                __buffer,
            )
        };
    }
    pub fn clear_acceleration_override(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_clear_acceleration_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_movement_mode_interface_clear_acceleration_override,
                __buffer,
            )
        };
    }
}
pub struct IChaosCharacterConstraintMovementModeInterface {}
#[repr(C, align(8))]
pub struct UChaosCharacterConstraintMovementModeInterface {
    __padding_end: [u8; 48],
}
impl UChaosCharacterConstraintMovementModeInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterConstraintMovementModeInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterConstraintMovementModeInterface")
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
pub struct UChaosCharacterMoverComponent {
    __padding_end: [u8; 2320],
}
impl UChaosCharacterMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterMoverComponent")
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
    pub fn override_movement_settings(
        &mut self,
        overrides: FChaosMovementSettingsOverrides,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_mover_component_override_movement_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &overrides,
                __buffer.add(0).cast::<FChaosMovementSettingsOverrides>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_mover_component_override_movement_settings,
                __buffer,
            )
        };
    }
    pub fn launch(
        &mut self,
        velocity_or_impulse: &crate::bindings::core_u_object::FVector,
        mode: EChaosMoverVelocityEffectMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_mover_component_launch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                velocity_or_impulse,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode,
                __buffer.add(24).cast::<EChaosMoverVelocityEffectMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_mover_component_launch,
                __buffer,
            )
        };
    }
    pub fn cancel_movement_settings_overrides(&mut self, mode_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_mover_component_cancel_movement_settings_overrides,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode_name,
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
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_character_mover_component_cancel_movement_settings_overrides,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UChaosCharacterMovementMode {
    __padding_end: [u8; 224],
}
impl UChaosCharacterMovementMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterMovementMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterMovementMode")
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
pub struct UChaosFallingMode {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub b_cancel_vertical_speed_on_landing: bool,
    pub air_control_percentage: f32,
    pub falling_deceleration: f32,
    pub falling_lateral_friction: f32,
    pub over_terminal_speed_falling_deceleration: f32,
    pub terminal_movement_plane_speed: f32,
    pub b_should_clamp_terminal_vertical_speed: bool,
    pub vertical_falling_deceleration: f32,
    pub terminal_vertical_speed: f32,
}
impl UChaosFallingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosFallingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosFallingMode")
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
pub struct UChaosFlyingMode {
    __padding_end: [u8; 224],
}
impl UChaosFlyingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosFlyingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosFlyingMode")
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
pub struct UChaosSwimmingMode {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub swimming_ideal_immersion_depth: f32,
    pub surface_swimming_water_control_settings: FSwimmingSettings,
    __padding_end: [u8; 8],
}
impl UChaosSwimmingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSwimmingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSwimmingMode")
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
pub struct UChaosWalkingMode {
    __padding_end: [u8; 248],
}
impl UChaosWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosWalkingMode")
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
pub struct UChaosCharacterCrouchCheck {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub b_cancel_on_mode_change: bool,
    pub capsule_half_height: TOptional<f32>,
    pub capsule_radius: TOptional<f32>,
    pub capsule_ground_clearance: TOptional<f32>,
    pub max_speed: TOptional<f32>,
    pub acceleration: TOptional<f32>,
    pub transition_to_crouched_mode: TOptional<FName>,
    pub transition_to_uncrouched_mode: TOptional<FName>,
    __padding_end: [u8; 20],
}
impl UChaosCharacterCrouchCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterCrouchCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterCrouchCheck")
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
pub struct UChaosCharacterFallingCheck {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub transition_to_falling_mode: FName,
    __padding_end: [u8; 12],
}
impl UChaosCharacterFallingCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterFallingCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterFallingCheck")
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
pub struct UChaosCharacterJumpCheck {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub jump_upwards_speed: f32,
    pub fractional_ground_reaction_impulse: f32,
    pub transition_to_mode: FName,
}
impl UChaosCharacterJumpCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterJumpCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterJumpCheck")
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
pub struct UChaosCharacterLandingCheck {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub floor_distance_tolerance: f32,
    pub transition_to_ground_mode: FName,
}
impl UChaosCharacterLandingCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterLandingCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterLandingCheck")
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
pub struct UChaosCharacterLaunchCheck {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub transition_to_mode: FName,
}
impl UChaosCharacterLaunchCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterLaunchCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterLaunchCheck")
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
pub struct UChaosCharacterWaterCheck {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub water_mode_start_immersion_depth: f32,
    pub water_mode_stop_immersion_depth: f32,
    pub water_mode_name: FName,
    pub ground_mode_name: FName,
    pub air_mode_name: FName,
}
impl UChaosCharacterWaterCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterWaterCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCharacterWaterCheck")
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
pub struct UChaosPathedMovementMode {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
    pub b_use_joint_constraint: bool,
    #[doc(hidden)]
    pub(crate) __padding_512: [u8; 356],
    pub one_way_playback_duration: f32,
    pub path_patterns: TArray<UPtr<UChaosPathedMovementPatternBase>>,
    pub easing: crate::bindings::engine::EAlphaBlendOption,
    pub custom_easing_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    __padding_end: [u8; 1000],
}
impl UChaosPathedMovementMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementMode")
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
    pub fn set_one_way_trip_duration_begin_play_only(&mut self, new_duration: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_mode_set_one_way_trip_duration_begin_play_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_duration,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_mode_set_one_way_trip_duration_begin_play_only,
                __buffer,
            )
        };
    }
    pub fn bp_find_pattern(
        &self,
        pattern_type: TSubclassOf<UChaosPathedMovementPatternBase>,
    ) -> UPtr<UChaosPathedMovementPatternBase> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_mode_bp_find_pattern,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pattern_type,
                __buffer.add(0).cast::<TSubclassOf<UChaosPathedMovementPatternBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_mode_bp_find_pattern,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UChaosPathedMovementPatternBase>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UChaosPathedMovementPatternBase {
    __padding_end: [u8; 96],
}
impl UChaosPathedMovementPatternBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementPatternBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementPatternBase")
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
pub struct UChaosEllipticalMovementPathPattern {
    __padding_end: [u8; 136],
}
impl UChaosEllipticalMovementPathPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosEllipticalMovementPathPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosEllipticalMovementPathPattern")
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
pub struct UChaosPathedMovementControllerComponent {
    __padding_end: [u8; 504],
}
impl UChaosPathedMovementControllerComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementControllerComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementControllerComponent")
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
    pub fn wants_reverse_playback(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_reverse_playback,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_reverse_playback,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn wants_playing_path(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_playing_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_playing_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn wants_one_way_playback(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_one_way_playback,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_one_way_playback,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn wants_looping_playback(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_looping_playback,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_wants_looping_playback,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn request_stop_playing_path(
        &mut self,
        execution_type: EChaosPathedMovementExecutionType,
        b_is_scheduled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_stop_playing_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &execution_type,
                __buffer.add(0).cast::<EChaosPathedMovementExecutionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_scheduled,
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
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_stop_playing_path,
                __buffer,
            )
        };
    }
    pub fn request_start_playing_path(
        &mut self,
        execution_type: EChaosPathedMovementExecutionType,
        b_is_scheduled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_start_playing_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &execution_type,
                __buffer.add(0).cast::<EChaosPathedMovementExecutionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_scheduled,
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
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_start_playing_path,
                __buffer,
            )
        };
    }
    pub fn request_reverse_playback(
        &mut self,
        b_wants_reverse_playback: bool,
        execution_type: EChaosPathedMovementExecutionType,
        b_is_scheduled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_reverse_playback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_wants_reverse_playback,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &execution_type,
                __buffer.add(1).cast::<EChaosPathedMovementExecutionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_scheduled,
                __buffer.add(2).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_reverse_playback,
                __buffer,
            )
        };
    }
    pub fn request_one_way_playback(
        &mut self,
        b_wants_one_way_playback: bool,
        execution_type: EChaosPathedMovementExecutionType,
        b_is_scheduled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_one_way_playback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_wants_one_way_playback,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &execution_type,
                __buffer.add(1).cast::<EChaosPathedMovementExecutionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_scheduled,
                __buffer.add(2).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_one_way_playback,
                __buffer,
            )
        };
    }
    pub fn request_looping_playback(
        &mut self,
        b_wants_looping_playback: bool,
        execution_type: EChaosPathedMovementExecutionType,
        b_is_scheduled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_looping_playback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_wants_looping_playback,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &execution_type,
                __buffer.add(1).cast::<EChaosPathedMovementExecutionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_scheduled,
                __buffer.add(2).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_request_looping_playback,
                __buffer,
            )
        };
    }
    pub fn is_reverse_playback(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_reverse_playback,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_reverse_playback,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_playing_path(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_playing_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_playing_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_one_way_playback(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_one_way_playback,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_one_way_playback,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_looping_playback(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_looping_playback,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_controller_component_is_looping_playback,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
pub struct IChaosPathedMovementDebugDrawInterface {}
#[repr(C, align(8))]
pub struct UChaosPathedMovementDebugDrawInterface {
    __padding_end: [u8; 48],
}
impl UChaosPathedMovementDebugDrawInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementDebugDrawInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementDebugDrawInterface")
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
    pub fn should_display_progress_preview_mesh(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_should_display_progress_preview_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_should_display_progress_preview_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_progress_preview_mesh_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_get_progress_preview_mesh_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_get_progress_preview_mesh_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_preview_mesh_overall_path_progress(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_get_preview_mesh_overall_path_progress,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_pathed_movement_debug_draw_interface_get_preview_mesh_overall_path_progress,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UChaosPathedMovementDebugDrawComponent {
    __padding_end: [u8; 1712],
}
impl UChaosPathedMovementDebugDrawComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementDebugDrawComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementDebugDrawComponent")
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
pub struct IChaosMovementActuationInterface {}
#[repr(C, align(8))]
pub struct UChaosMovementActuationInterface {
    __padding_end: [u8; 48],
}
impl UChaosMovementActuationInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMovementActuationInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosMovementActuationInterface")
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
pub struct IChaosPathedMovementModeInterface {}
#[repr(C, align(8))]
pub struct UChaosPathedMovementModeInterface {
    __padding_end: [u8; 48],
}
impl UChaosPathedMovementModeInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementModeInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementModeInterface")
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
pub struct UChaosPointMovementPathPattern {
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 96],
    pub path_points: TArray<FChaosPointMovementPathPoint>,
    __padding_end: [u8; 112],
}
impl UChaosPointMovementPathPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPointMovementPathPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPointMovementPathPattern")
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
pub struct UChaosSplineMovementPathPattern {
    __padding_end: [u8; 168],
}
impl UChaosSplineMovementPathPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSplineMovementPathPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSplineMovementPathPattern")
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
pub struct UChaosPathedMovementReachedEndTransition {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub transition_to_mode: FName,
}
impl UChaosPathedMovementReachedEndTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementReachedEndTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosPathedMovementReachedEndTransition")
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
pub struct UChaosGroundMovementUtils {
    __padding_end: [u8; 48],
}
impl UChaosGroundMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosGroundMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosGroundMovementUtils")
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
    pub fn compute_local_ground_velocity_internal(
        position: &crate::bindings::core_u_object::FVector,
        floor_result: &crate::bindings::mover::FFloorCheckResult,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<328>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_ground_movement_utils_compute_local_ground_velocity_internal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                floor_result,
                __buffer.add(24).cast::<crate::bindings::mover::FFloorCheckResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_mover::UChaosGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_mover::__FUNCTION_PTRS
                    .u_chaos_ground_movement_utils_compute_local_ground_velocity_internal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(304).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct FChaosCharacterMoverComponent_OnLanded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosCharacterMoverComponent_OnJumped {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosPathedMovementControllerComponent_OnPathedMovementStarted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosPathedMovementControllerComponent_OnPathedMovementStopped {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosPathedMovementControllerComponent_OnPathedMovementBounced {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosPathedMovementControllerComponent_OnPathedMovementReversePlaybackChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosPathedMovementControllerComponent_OnPathedMovementLoopingPlaybackChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosPathedMovementControllerComponent_OnPathedMovementOneWayPlaybackChanged {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EChaosMoverVelocityEffectMode(pub u8);
impl EChaosMoverVelocityEffectMode {
    pub const IMPULSE: EChaosMoverVelocityEffectMode = EChaosMoverVelocityEffectMode(0);
    pub const ADDITIVE_VELOCITY: EChaosMoverVelocityEffectMode = EChaosMoverVelocityEffectMode(
        1,
    );
    pub const OVERRIDE_VELOCITY: EChaosMoverVelocityEffectMode = EChaosMoverVelocityEffectMode(
        2,
    );
}
#[repr(transparent)]
pub struct EChaosPointMovementLocationBasis(pub u8);
impl EChaosPointMovementLocationBasis {
    pub const PREVIOUS_POINT: EChaosPointMovementLocationBasis = EChaosPointMovementLocationBasis(
        0,
    );
    pub const PATH_ORIGIN: EChaosPointMovementLocationBasis = EChaosPointMovementLocationBasis(
        1,
    );
    pub const WORLD: EChaosPointMovementLocationBasis = EChaosPointMovementLocationBasis(
        2,
    );
}
#[repr(transparent)]
pub struct EChaosPathedMovementExecutionType(pub u8);
impl EChaosPathedMovementExecutionType {
    pub const AUTHORITY_ONLY: EChaosPathedMovementExecutionType = EChaosPathedMovementExecutionType(
        0,
    );
    pub const CLIENT_PREDICTED_AUTONOMOUS_ONLY: EChaosPathedMovementExecutionType = EChaosPathedMovementExecutionType(
        1,
    );
}
#[repr(transparent)]
pub struct EChaosMoverIgnoredCollisionMode(pub u8);
impl EChaosMoverIgnoredCollisionMode {
    pub const ENABLE_COLLISIONS_WITH_IGNORED: EChaosMoverIgnoredCollisionMode = EChaosMoverIgnoredCollisionMode(
        0,
    );
    pub const DISABLE_COLLISIONS_WITH_IGNORED: EChaosMoverIgnoredCollisionMode = EChaosMoverIgnoredCollisionMode(
        1,
    );
}
#[repr(transparent)]
pub struct ECharacterMoverFrictionOverrideMode(pub u8);
impl ECharacterMoverFrictionOverrideMode {
    pub const DO_NOT_OVERRIDE: ECharacterMoverFrictionOverrideMode = ECharacterMoverFrictionOverrideMode(
        0,
    );
    pub const ALWAYS_OVERRIDE_TO_ZERO: ECharacterMoverFrictionOverrideMode = ECharacterMoverFrictionOverrideMode(
        1,
    );
    pub const OVERRIDE_TO_ZERO_WHEN_MOVING: ECharacterMoverFrictionOverrideMode = ECharacterMoverFrictionOverrideMode(
        2,
    );
}
