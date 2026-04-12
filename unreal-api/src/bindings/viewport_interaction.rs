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
    pub u_viewport_interactor_tick: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_shutdown: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_set_hit_result_gizmo_filter_mode: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_set_dragging_mode: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_set_can_carry: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_is_hovering_over_gizmo: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_handle_input_key_bp: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_handle_input_axis_bp: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_world_interaction: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_transform_and_forward_vector: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_room_space_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_other_interactor: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_last_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_last_room_space_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_laser_pointer: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_hover_location: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_hit_result_gizmo_filter_mode: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_get_dragging_mode: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_interactor_can_carry: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_transformer_shutdown: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_transformer_should_center_transform_gizmo_pivot: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_transformer_on_stop_dragging: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_transformer_on_start_dragging: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_transformer_init: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_transformer_can_align_to_actors: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_set_world_to_meters_scale: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_set_room_transform_for_next_frame: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_set_head_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_remove_interactor: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_get_world_scale_factor: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_get_transform_gizmo_actor: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_get_room_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_get_room_space_head_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_get_interactors: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_get_head_transform: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_add_interactor: *mut crate::ffi::UFunctionOpague,
    pub u_viewport_world_interaction_add_actor_to_exclude_from_hit_tests: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_viewport_interactor_tick: std::ptr::null_mut(),
            u_viewport_interactor_shutdown: std::ptr::null_mut(),
            u_viewport_interactor_set_hit_result_gizmo_filter_mode: std::ptr::null_mut(),
            u_viewport_interactor_set_dragging_mode: std::ptr::null_mut(),
            u_viewport_interactor_set_can_carry: std::ptr::null_mut(),
            u_viewport_interactor_is_hovering_over_gizmo: std::ptr::null_mut(),
            u_viewport_interactor_handle_input_key_bp: std::ptr::null_mut(),
            u_viewport_interactor_handle_input_axis_bp: std::ptr::null_mut(),
            u_viewport_interactor_get_world_interaction: std::ptr::null_mut(),
            u_viewport_interactor_get_transform_and_forward_vector: std::ptr::null_mut(),
            u_viewport_interactor_get_transform: std::ptr::null_mut(),
            u_viewport_interactor_get_room_space_transform: std::ptr::null_mut(),
            u_viewport_interactor_get_other_interactor: std::ptr::null_mut(),
            u_viewport_interactor_get_last_transform: std::ptr::null_mut(),
            u_viewport_interactor_get_last_room_space_transform: std::ptr::null_mut(),
            u_viewport_interactor_get_laser_pointer: std::ptr::null_mut(),
            u_viewport_interactor_get_hover_location: std::ptr::null_mut(),
            u_viewport_interactor_get_hit_result_gizmo_filter_mode: std::ptr::null_mut(),
            u_viewport_interactor_get_dragging_mode: std::ptr::null_mut(),
            u_viewport_interactor_can_carry: std::ptr::null_mut(),
            u_viewport_transformer_shutdown: std::ptr::null_mut(),
            u_viewport_transformer_should_center_transform_gizmo_pivot: std::ptr::null_mut(),
            u_viewport_transformer_on_stop_dragging: std::ptr::null_mut(),
            u_viewport_transformer_on_start_dragging: std::ptr::null_mut(),
            u_viewport_transformer_init: std::ptr::null_mut(),
            u_viewport_transformer_can_align_to_actors: std::ptr::null_mut(),
            u_viewport_world_interaction_set_world_to_meters_scale: std::ptr::null_mut(),
            u_viewport_world_interaction_set_room_transform_for_next_frame: std::ptr::null_mut(),
            u_viewport_world_interaction_set_head_transform: std::ptr::null_mut(),
            u_viewport_world_interaction_remove_interactor: std::ptr::null_mut(),
            u_viewport_world_interaction_get_world_scale_factor: std::ptr::null_mut(),
            u_viewport_world_interaction_get_transform_gizmo_actor: std::ptr::null_mut(),
            u_viewport_world_interaction_get_room_transform: std::ptr::null_mut(),
            u_viewport_world_interaction_get_room_space_head_transform: std::ptr::null_mut(),
            u_viewport_world_interaction_get_interactors: std::ptr::null_mut(),
            u_viewport_world_interaction_get_head_transform: std::ptr::null_mut(),
            u_viewport_world_interaction_add_interactor: std::ptr::null_mut(),
            u_viewport_world_interaction_add_actor_to_exclude_from_hit_tests: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UViewportInteractor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Tick"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Shutdown"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_shutdown,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHitResultGizmoFilterMode"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_interactor_set_hit_result_gizmo_filter_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDraggingMode"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_set_dragging_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCanCarry"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_set_can_carry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHoveringOverGizmo"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_is_hovering_over_gizmo,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleInputKey_BP"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_handle_input_key_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleInputAxis_BP"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_handle_input_axis_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWorldInteraction"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_world_interaction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformAndForwardVector"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_interactor_get_transform_and_forward_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransform"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRoomSpaceTransform"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_room_space_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOtherInteractor"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_other_interactor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastTransform"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_last_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastRoomSpaceTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_interactor_get_last_room_space_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLaserPointer"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_laser_pointer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHoverLocation"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_hover_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHitResultGizmoFilterMode"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_interactor_get_hit_result_gizmo_filter_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDraggingMode"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_get_dragging_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanCarry"),
                &raw mut __FUNCTION_PTRS.u_viewport_interactor_can_carry,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UViewportTransformer::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Shutdown"),
                &raw mut __FUNCTION_PTRS.u_viewport_transformer_shutdown,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldCenterTransformGizmoPivot"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_transformer_should_center_transform_gizmo_pivot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnStopDragging"),
                &raw mut __FUNCTION_PTRS.u_viewport_transformer_on_stop_dragging,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnStartDragging"),
                &raw mut __FUNCTION_PTRS.u_viewport_transformer_on_start_dragging,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Init"),
                &raw mut __FUNCTION_PTRS.u_viewport_transformer_init,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanAlignToActors"),
                &raw mut __FUNCTION_PTRS.u_viewport_transformer_can_align_to_actors,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UViewportWorldInteraction::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWorldToMetersScale"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_world_interaction_set_world_to_meters_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRoomTransformForNextFrame"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_world_interaction_set_room_transform_for_next_frame,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHeadTransform"),
                &raw mut __FUNCTION_PTRS.u_viewport_world_interaction_set_head_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveInteractor"),
                &raw mut __FUNCTION_PTRS.u_viewport_world_interaction_remove_interactor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWorldScaleFactor"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_world_interaction_get_world_scale_factor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformGizmoActor"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_world_interaction_get_transform_gizmo_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRoomTransform"),
                &raw mut __FUNCTION_PTRS.u_viewport_world_interaction_get_room_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRoomSpaceHeadTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_world_interaction_get_room_space_head_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInteractors"),
                &raw mut __FUNCTION_PTRS.u_viewport_world_interaction_get_interactors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHeadTransform"),
                &raw mut __FUNCTION_PTRS.u_viewport_world_interaction_get_head_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInteractor"),
                &raw mut __FUNCTION_PTRS.u_viewport_world_interaction_add_interactor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddActorToExcludeFromHitTests"),
                &raw mut __FUNCTION_PTRS
                    .u_viewport_world_interaction_add_actor_to_exclude_from_hit_tests,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FViewportActionKeyInput {
    pub action_type: FName,
    pub event: crate::bindings::engine::EInputEvent,
}
impl FViewportActionKeyInput {}
#[repr(C, align(8))]
pub struct UViewportDragOperation {
    __padding_end: [u8; 56],
}
impl UViewportDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportDragOperation")
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
pub struct UViewportInteractor {
    __padding_end: [u8; 1952],
}
impl UViewportInteractor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractor")
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
    pub fn tick(&mut self, delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_tick,
                __buffer,
            )
        };
        std::mem::forget(delta_time);
    }
    pub fn shutdown(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_shutdown,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_shutdown,
                __buffer,
            )
        };
    }
    pub fn set_hit_result_gizmo_filter_mode(
        &mut self,
        new_filter: EHitResultGizmoFilterMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_set_hit_result_gizmo_filter_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_filter,
                __buffer.add(0).cast::<EHitResultGizmoFilterMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_set_hit_result_gizmo_filter_mode,
                __buffer,
            )
        };
        std::mem::forget(new_filter);
    }
    pub fn set_dragging_mode(
        &mut self,
        new_dragging_mode: EViewportInteractionDraggingMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_set_dragging_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_dragging_mode,
                __buffer.add(0).cast::<EViewportInteractionDraggingMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_set_dragging_mode,
                __buffer,
            )
        };
        std::mem::forget(new_dragging_mode);
    }
    pub fn set_can_carry(&mut self, b_in_can_carry: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_set_can_carry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_can_carry,
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
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_set_can_carry,
                __buffer,
            )
        };
        std::mem::forget(b_in_can_carry);
    }
    pub fn is_hovering_over_gizmo(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_is_hovering_over_gizmo,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_is_hovering_over_gizmo,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn handle_input_key_bp(
        &mut self,
        action: &FViewportActionKeyInput,
        key: crate::bindings::input_core::FKey,
        event: crate::bindings::engine::EInputEvent,
        b_out_was_handled: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_handle_input_key_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                action,
                __buffer.add(0).cast::<FViewportActionKeyInput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(16).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event,
                __buffer.add(48).cast::<crate::bindings::engine::EInputEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_was_handled,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_handle_input_key_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(49).cast::<bool>().swap(b_out_was_handled);
        }
        std::mem::forget(key);
        std::mem::forget(event);
    }
    pub fn handle_input_axis_bp(
        &mut self,
        action: &FViewportActionKeyInput,
        key: crate::bindings::input_core::FKey,
        delta: f32,
        delta_time: f32,
        b_out_was_handled: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_handle_input_axis_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                action,
                __buffer.add(0).cast::<FViewportActionKeyInput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(16).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&delta, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_time,
                __buffer.add(52).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_was_handled,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_handle_input_axis_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<bool>().swap(b_out_was_handled);
        }
        std::mem::forget(key);
        std::mem::forget(delta);
        std::mem::forget(delta_time);
    }
    pub fn get_world_interaction(&self) -> UPtr<UViewportWorldInteraction> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_world_interaction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_world_interaction,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UViewportWorldInteraction>>().read() }
    }
    pub fn get_transform_and_forward_vector(
        &self,
        out_hand_transform: &mut crate::bindings::core_u_object::FTransform,
        out_forward_vector: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<121>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_transform_and_forward_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hand_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_forward_vector,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_transform_and_forward_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(out_hand_transform);
        }
        unsafe {
            __buffer
                .add(96)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_forward_vector);
        }
        unsafe { __buffer.add(120).cast::<bool>().read() }
    }
    pub fn get_transform(&self) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_room_space_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_room_space_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_room_space_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_other_interactor(&self) -> UPtr<UViewportInteractor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_other_interactor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_other_interactor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UViewportInteractor>>().read() }
    }
    pub fn get_last_transform(&self) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_last_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_last_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_last_room_space_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_last_room_space_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_last_room_space_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_laser_pointer(
        &mut self,
        laser_pointer_start: &mut crate::bindings::core_u_object::FVector,
        laser_pointer_end: &mut crate::bindings::core_u_object::FVector,
        b_even_if_blocked: bool,
        laser_length_override: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_laser_pointer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                laser_pointer_start,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                laser_pointer_end,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_even_if_blocked,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &laser_length_override,
                __buffer.add(52).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_laser_pointer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(laser_pointer_start);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(laser_pointer_end);
        }
        std::mem::forget(b_even_if_blocked);
        std::mem::forget(laser_length_override);
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn get_hover_location(&mut self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_hover_location,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_hover_location,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_hit_result_gizmo_filter_mode(&self) -> EHitResultGizmoFilterMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_hit_result_gizmo_filter_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_hit_result_gizmo_filter_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EHitResultGizmoFilterMode>().read() }
    }
    pub fn get_dragging_mode(&self) -> EViewportInteractionDraggingMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_dragging_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_get_dragging_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EViewportInteractionDraggingMode>().read() }
    }
    pub fn can_carry(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_can_carry,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_interactor_can_carry,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UVISettings {
    __padding_end: [u8; 56],
}
impl UVISettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVISettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVISettings")
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
pub struct UViewportInteractionAssetContainer {
    __padding_end: [u8; 232],
}
impl UViewportInteractionAssetContainer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractionAssetContainer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractionAssetContainer")
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
pub struct UViewportTransformer {
    __padding_end: [u8; 56],
}
impl UViewportTransformer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportTransformer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportTransformer")
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
pub struct UActorTransformer {
    __padding_end: [u8; 56],
}
impl UActorTransformer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorTransformer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorTransformer")
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
pub struct ABaseTransformGizmo {
    __padding_end: [u8; 1184],
}
impl ABaseTransformGizmo {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ABaseTransformGizmo")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ABaseTransformGizmo")
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
pub struct UGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGizmoHandleGroup")
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
pub struct UAxisGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UAxisGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAxisGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAxisGizmoHandleGroup")
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
pub struct UGizmoHandleMeshComponent {
    __padding_end: [u8; 1888],
}
impl UGizmoHandleMeshComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGizmoHandleMeshComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGizmoHandleMeshComponent")
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
pub struct APivotTransformGizmo {
    __padding_end: [u8; 1248],
}
impl APivotTransformGizmo {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APivotTransformGizmo")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APivotTransformGizmo")
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
pub struct UPivotTranslationGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotTranslationGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotTranslationGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotTranslationGizmoHandleGroup")
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
pub struct UPivotScaleGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotScaleGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotScaleGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotScaleGizmoHandleGroup")
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
pub struct UPivotPlaneTranslationGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotPlaneTranslationGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotPlaneTranslationGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotPlaneTranslationGizmoHandleGroup")
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
pub struct UPivotRotationGizmoHandleGroup {
    __padding_end: [u8; 816],
}
impl UPivotRotationGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotRotationGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotRotationGizmoHandleGroup")
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
pub struct UStretchGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UStretchGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchGizmoHandleGroup")
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
pub struct UStretchGizmoHandleDragOperation {
    __padding_end: [u8; 56],
}
impl UStretchGizmoHandleDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchGizmoHandleDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchGizmoHandleDragOperation")
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
pub struct UUniformScaleGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UUniformScaleGizmoHandleGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScaleGizmoHandleGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScaleGizmoHandleGroup")
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
pub struct UMouseCursorInteractor {
    __padding_end: [u8; 1952],
}
impl UMouseCursorInteractor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMouseCursorInteractor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMouseCursorInteractor")
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
pub struct UViewportDragOperationComponent {
    __padding_end: [u8; 256],
}
impl UViewportDragOperationComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportDragOperationComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportDragOperationComponent")
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
pub struct IViewportInteractableInterface {}
#[repr(C, align(8))]
pub struct UViewportInteractableInterface {
    __padding_end: [u8; 48],
}
impl UViewportInteractableInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractableInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractableInterface")
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
pub struct UTranslationDragOperation {
    __padding_end: [u8; 56],
}
impl UTranslationDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTranslationDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTranslationDragOperation")
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
pub struct UPlaneTranslationDragOperation {
    __padding_end: [u8; 56],
}
impl UPlaneTranslationDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneTranslationDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneTranslationDragOperation")
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
pub struct URotateOnAngleDragOperation {
    __padding_end: [u8; 120],
}
impl URotateOnAngleDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotateOnAngleDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotateOnAngleDragOperation")
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
pub struct UScaleDragOperation {
    __padding_end: [u8; 56],
}
impl UScaleDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleDragOperation")
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
pub struct UUniformScaleDragOperation {
    __padding_end: [u8; 56],
}
impl UUniformScaleDragOperation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScaleDragOperation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScaleDragOperation")
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
pub struct UViewportWorldInteraction {
    __padding_end: [u8; 1376],
}
impl UViewportWorldInteraction {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportWorldInteraction")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportWorldInteraction")
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
    pub fn set_world_to_meters_scale(
        &mut self,
        new_world_to_meters_scale: f32,
        b_compensate_room_world_scale: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_set_world_to_meters_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_world_to_meters_scale,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_compensate_room_world_scale,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_set_world_to_meters_scale,
                __buffer,
            )
        };
        std::mem::forget(new_world_to_meters_scale);
        std::mem::forget(b_compensate_room_world_scale);
    }
    pub fn set_room_transform_for_next_frame(
        &mut self,
        new_room_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_set_room_transform_for_next_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_room_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_set_room_transform_for_next_frame,
                __buffer,
            )
        };
    }
    pub fn set_head_transform(
        &mut self,
        new_head_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_set_head_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_head_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_set_head_transform,
                __buffer,
            )
        };
    }
    pub fn remove_interactor(&mut self, interactor: UPtr<UViewportInteractor>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_remove_interactor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interactor,
                __buffer.add(0).cast::<UPtr<UViewportInteractor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_remove_interactor,
                __buffer,
            )
        };
        std::mem::forget(interactor);
    }
    pub fn get_world_scale_factor(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_world_scale_factor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_world_scale_factor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_transform_gizmo_actor(&mut self) -> UPtr<ABaseTransformGizmo> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_transform_gizmo_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_transform_gizmo_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ABaseTransformGizmo>>().read() }
    }
    pub fn get_room_transform(&self) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_room_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_room_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_room_space_head_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_room_space_head_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_room_space_head_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_interactors(&self) -> TArray<UPtr<UViewportInteractor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_interactors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_interactors,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UViewportInteractor>>>().read() }
    }
    pub fn get_head_transform(&self) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_head_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_get_head_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn add_interactor(&mut self, interactor: UPtr<UViewportInteractor>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_add_interactor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interactor,
                __buffer.add(0).cast::<UPtr<UViewportInteractor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_add_interactor,
                __buffer,
            )
        };
        std::mem::forget(interactor);
    }
    pub fn add_actor_to_exclude_from_hit_tests(
        &mut self,
        actor_to_exclude_from_hit_tests: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_add_actor_to_exclude_from_hit_tests,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_exclude_from_hit_tests,
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
                crate::bindings::viewport_interaction::__FUNCTION_PTRS
                    .u_viewport_world_interaction_add_actor_to_exclude_from_hit_tests,
                __buffer,
            )
        };
        std::mem::forget(actor_to_exclude_from_hit_tests);
    }
}
#[repr(transparent)]
pub struct EViewportInteractionDraggingMode(pub u8);
impl EViewportInteractionDraggingMode {
    pub const NOTHING: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        0,
    );
    pub const TRANSFORMABLES_WITH_GIZMO: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        1,
    );
    pub const TRANSFORMABLES_AT_LASER_IMPACT: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        2,
    );
    pub const ASSISTING_DRAG: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        3,
    );
    pub const TRANSFORMABLES_FREELY: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        4,
    );
    pub const WORLD: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        5,
    );
    pub const INTERACTABLE: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        6,
    );
    pub const MATERIAL: EViewportInteractionDraggingMode = EViewportInteractionDraggingMode(
        7,
    );
}
#[repr(transparent)]
pub struct EHitResultGizmoFilterMode(pub u8);
impl EHitResultGizmoFilterMode {
    pub const ALL: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(0);
    pub const NO_GIZMOS: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(1);
    pub const GIZMOS_ONLY: EHitResultGizmoFilterMode = EHitResultGizmoFilterMode(2);
}
#[repr(transparent)]
pub struct EGizmoHandleTypes(pub u8);
impl EGizmoHandleTypes {
    pub const ALL: EGizmoHandleTypes = EGizmoHandleTypes(0);
    pub const TRANSLATE: EGizmoHandleTypes = EGizmoHandleTypes(1);
    pub const ROTATE: EGizmoHandleTypes = EGizmoHandleTypes(2);
    pub const SCALE: EGizmoHandleTypes = EGizmoHandleTypes(3);
}
