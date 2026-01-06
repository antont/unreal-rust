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
pub static mut U_VIEWPORT_INTERACTOR_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_SHUTDOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_SET_HIT_RESULT_GIZMO_FILTER_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_SET_DRAGGING_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_SET_CAN_CARRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_IS_HOVERING_OVER_GIZMO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_HANDLE_INPUT_KEY_BP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_HANDLE_INPUT_AXIS_BP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_WORLD_INTERACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_TRANSFORM_AND_FORWARD_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_ROOM_SPACE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_OTHER_INTERACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_LAST_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_LAST_ROOM_SPACE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_LASER_POINTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_HOVER_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_HIT_RESULT_GIZMO_FILTER_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_GET_DRAGGING_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_INTERACTOR_CAN_CARRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_TRANSFORMER_SHUTDOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_TRANSFORMER_SHOULD_CENTER_TRANSFORM_GIZMO_PIVOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_TRANSFORMER_ON_STOP_DRAGGING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_TRANSFORMER_ON_START_DRAGGING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_TRANSFORMER_INIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_TRANSFORMER_CAN_ALIGN_TO_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_SET_WORLD_TO_METERS_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_SET_ROOM_TRANSFORM_FOR_NEXT_FRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_SET_HEAD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_REMOVE_INTERACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_GET_WORLD_SCALE_FACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_GET_TRANSFORM_GIZMO_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_GET_ROOM_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_GET_ROOM_SPACE_HEAD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_GET_INTERACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_GET_HEAD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_ADD_INTERACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_WORLD_INTERACTION_ADD_ACTOR_TO_EXCLUDE_FROM_HIT_TESTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UViewportInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Tick"),
            &raw mut U_VIEWPORT_INTERACTOR_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Shutdown"),
            &raw mut U_VIEWPORT_INTERACTOR_SHUTDOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHitResultGizmoFilterMode"),
            &raw mut U_VIEWPORT_INTERACTOR_SET_HIT_RESULT_GIZMO_FILTER_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDraggingMode"),
            &raw mut U_VIEWPORT_INTERACTOR_SET_DRAGGING_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCanCarry"),
            &raw mut U_VIEWPORT_INTERACTOR_SET_CAN_CARRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsHoveringOverGizmo"),
            &raw mut U_VIEWPORT_INTERACTOR_IS_HOVERING_OVER_GIZMO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HandleInputKey_BP"),
            &raw mut U_VIEWPORT_INTERACTOR_HANDLE_INPUT_KEY_BP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HandleInputAxis_BP"),
            &raw mut U_VIEWPORT_INTERACTOR_HANDLE_INPUT_AXIS_BP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorldInteraction"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_WORLD_INTERACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTransformAndForwardVector"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_TRANSFORM_AND_FORWARD_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTransform"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoomSpaceTransform"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_ROOM_SPACE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOtherInteractor"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_OTHER_INTERACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastTransform"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_LAST_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastRoomSpaceTransform"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_LAST_ROOM_SPACE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLaserPointer"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_LASER_POINTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHoverLocation"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_HOVER_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHitResultGizmoFilterMode"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_HIT_RESULT_GIZMO_FILTER_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDraggingMode"),
            &raw mut U_VIEWPORT_INTERACTOR_GET_DRAGGING_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanCarry"),
            &raw mut U_VIEWPORT_INTERACTOR_CAN_CARRY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UViewportTransformer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Shutdown"),
            &raw mut U_VIEWPORT_TRANSFORMER_SHUTDOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldCenterTransformGizmoPivot"),
            &raw mut U_VIEWPORT_TRANSFORMER_SHOULD_CENTER_TRANSFORM_GIZMO_PIVOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnStopDragging"),
            &raw mut U_VIEWPORT_TRANSFORMER_ON_STOP_DRAGGING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnStartDragging"),
            &raw mut U_VIEWPORT_TRANSFORMER_ON_START_DRAGGING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Init"),
            &raw mut U_VIEWPORT_TRANSFORMER_INIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanAlignToActors"),
            &raw mut U_VIEWPORT_TRANSFORMER_CAN_ALIGN_TO_ACTORS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UViewportWorldInteraction::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWorldToMetersScale"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_SET_WORLD_TO_METERS_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRoomTransformForNextFrame"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_SET_ROOM_TRANSFORM_FOR_NEXT_FRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHeadTransform"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_SET_HEAD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveInteractor"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_REMOVE_INTERACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorldScaleFactor"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_GET_WORLD_SCALE_FACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTransformGizmoActor"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_GET_TRANSFORM_GIZMO_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoomTransform"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_GET_ROOM_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoomSpaceHeadTransform"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_GET_ROOM_SPACE_HEAD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInteractors"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_GET_INTERACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHeadTransform"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_GET_HEAD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInteractor"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_ADD_INTERACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorToExcludeFromHitTests"),
            &raw mut U_VIEWPORT_WORLD_INTERACTION_ADD_ACTOR_TO_EXCLUDE_FROM_HIT_TESTS,
        );
    }
}
#[repr(C, align(4))]
pub struct FViewportActionKeyInput {
    pub action_type: FName,
    pub event: crate::bindings::engine::EInputEvent,
    __padding_end: [u8; 3],
}
impl FViewportActionKeyInput {}
#[repr(C, align(8))]
pub struct UViewportDragOperation {
    __padding_end: [u8; 56],
}
impl UViewportDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportDragOperation")
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
pub struct UViewportInteractor {
    __padding_end: [u8; 1952],
}
impl UViewportInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractor")
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
pub struct UVISettings {
    __padding_end: [u8; 56],
}
impl UVISettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVISettings")
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
pub struct UViewportInteractionAssetContainer {
    __padding_end: [u8; 232],
}
impl UViewportInteractionAssetContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractionAssetContainer")
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
pub struct UViewportTransformer {
    __padding_end: [u8; 56],
}
impl UViewportTransformer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportTransformer")
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
pub struct UActorTransformer {
    __padding_end: [u8; 56],
}
impl UActorTransformer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorTransformer")
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
pub struct ABaseTransformGizmo {
    __padding_end: [u8; 1184],
}
impl ABaseTransformGizmo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ABaseTransformGizmo")
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
pub struct UGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGizmoHandleGroup")
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
pub struct UAxisGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UAxisGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAxisGizmoHandleGroup")
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
pub struct UGizmoHandleMeshComponent {
    __padding_end: [u8; 1888],
}
impl UGizmoHandleMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGizmoHandleMeshComponent")
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
pub struct APivotTransformGizmo {
    __padding_end: [u8; 1248],
}
impl APivotTransformGizmo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APivotTransformGizmo")
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
pub struct UPivotTranslationGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotTranslationGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotTranslationGizmoHandleGroup")
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
pub struct UPivotScaleGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotScaleGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotScaleGizmoHandleGroup")
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
pub struct UPivotPlaneTranslationGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UPivotPlaneTranslationGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotPlaneTranslationGizmoHandleGroup")
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
pub struct UPivotRotationGizmoHandleGroup {
    __padding_end: [u8; 816],
}
impl UPivotRotationGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotRotationGizmoHandleGroup")
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
pub struct UStretchGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UStretchGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchGizmoHandleGroup")
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
pub struct UStretchGizmoHandleDragOperation {
    __padding_end: [u8; 56],
}
impl UStretchGizmoHandleDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchGizmoHandleDragOperation")
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
pub struct UUniformScaleGizmoHandleGroup {
    __padding_end: [u8; 720],
}
impl UUniformScaleGizmoHandleGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScaleGizmoHandleGroup")
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
pub struct UMouseCursorInteractor {
    __padding_end: [u8; 1952],
}
impl UMouseCursorInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMouseCursorInteractor")
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
pub struct UViewportDragOperationComponent {
    __padding_end: [u8; 256],
}
impl UViewportDragOperationComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportDragOperationComponent")
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
pub struct IViewportInteractableInterface {}
#[repr(C, align(8))]
pub struct UViewportInteractableInterface {
    __padding_end: [u8; 48],
}
impl UViewportInteractableInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportInteractableInterface")
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
pub struct UTranslationDragOperation {
    __padding_end: [u8; 56],
}
impl UTranslationDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTranslationDragOperation")
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
pub struct UPlaneTranslationDragOperation {
    __padding_end: [u8; 56],
}
impl UPlaneTranslationDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneTranslationDragOperation")
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
pub struct URotateOnAngleDragOperation {
    __padding_end: [u8; 120],
}
impl URotateOnAngleDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotateOnAngleDragOperation")
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
pub struct UScaleDragOperation {
    __padding_end: [u8; 56],
}
impl UScaleDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleDragOperation")
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
pub struct UUniformScaleDragOperation {
    __padding_end: [u8; 56],
}
impl UUniformScaleDragOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScaleDragOperation")
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
pub struct UViewportWorldInteraction {
    __padding_end: [u8; 1376],
}
impl UViewportWorldInteraction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportWorldInteraction")
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
