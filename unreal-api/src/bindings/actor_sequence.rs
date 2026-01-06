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
pub static mut U_ACTOR_SEQUENCE_COMPONENT_STOP_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTOR_SEQUENCE_COMPONENT_PLAY_SEQUENCE_REVERSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTOR_SEQUENCE_COMPONENT_PLAY_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTOR_SEQUENCE_COMPONENT_PAUSE_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UActorSequenceComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopSequence"),
            &raw mut U_ACTOR_SEQUENCE_COMPONENT_STOP_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlaySequenceReverse"),
            &raw mut U_ACTOR_SEQUENCE_COMPONENT_PLAY_SEQUENCE_REVERSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlaySequence"),
            &raw mut U_ACTOR_SEQUENCE_COMPONENT_PLAY_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PauseSequence"),
            &raw mut U_ACTOR_SEQUENCE_COMPONENT_PAUSE_SEQUENCE,
        );
    }
}
#[repr(C, align(8))]
pub struct UActorSequence {
    __padding_end: [u8; 176],
}
impl UActorSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequence")
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
pub struct UActorSequenceComponent {
    #[doc(hidden)]
    __padding_288: [u8; 288],
    pub sequence_player: UPtr<UActorSequencePlayer>,
}
impl UActorSequenceComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequenceComponent")
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
pub struct UActorSequencePlayer {
    __padding_end: [u8; 1216],
}
impl UActorSequencePlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorSequencePlayer")
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
