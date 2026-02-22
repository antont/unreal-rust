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
    pub u_actor_sequence_component_stop_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_actor_sequence_component_play_sequence_reverse: *mut crate::ffi::UFunctionOpague,
    pub u_actor_sequence_component_play_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_actor_sequence_component_pause_sequence: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_actor_sequence_component_stop_sequence: std::ptr::null_mut(),
            u_actor_sequence_component_play_sequence_reverse: std::ptr::null_mut(),
            u_actor_sequence_component_play_sequence: std::ptr::null_mut(),
            u_actor_sequence_component_pause_sequence: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UActorSequenceComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopSequence"),
                &raw mut __FUNCTION_PTRS.u_actor_sequence_component_stop_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlaySequenceReverse"),
                &raw mut __FUNCTION_PTRS.u_actor_sequence_component_play_sequence_reverse,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlaySequence"),
                &raw mut __FUNCTION_PTRS.u_actor_sequence_component_play_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PauseSequence"),
                &raw mut __FUNCTION_PTRS.u_actor_sequence_component_pause_sequence,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UActorSequence {
    __padding_end: [u8; 176],
}
impl UActorSequence {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequence")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequence")
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
pub struct UActorSequenceComponent {
    #[doc(hidden)]
    pub(crate) __padding_288: [u8; 288],
    pub sequence_player: UPtr<UActorSequencePlayer>,
}
impl UActorSequenceComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequenceComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequenceComponent")
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
    pub fn stop_sequence(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_stop_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_stop_sequence,
                __buffer,
            )
        };
    }
    pub fn play_sequence_reverse(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_play_sequence_reverse,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_play_sequence_reverse,
                __buffer,
            )
        };
    }
    pub fn play_sequence(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_play_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_play_sequence,
                __buffer,
            )
        };
    }
    pub fn pause_sequence(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_pause_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_sequence::__FUNCTION_PTRS
                    .u_actor_sequence_component_pause_sequence,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UActorSequencePlayer {
    __padding_end: [u8; 1216],
}
impl UActorSequencePlayer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequencePlayer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequencePlayer")
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
#[repr(transparent)]
pub struct EActorSequenceObjectReferenceType(pub u8);
impl EActorSequenceObjectReferenceType {
    pub const CONTEXT_ACTOR: EActorSequenceObjectReferenceType = EActorSequenceObjectReferenceType(
        0,
    );
    pub const EXTERNAL_ACTOR: EActorSequenceObjectReferenceType = EActorSequenceObjectReferenceType(
        1,
    );
    pub const COMPONENT: EActorSequenceObjectReferenceType = EActorSequenceObjectReferenceType(
        2,
    );
}
