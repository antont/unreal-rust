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
pub static mut U_TURN_BASED_MATCH_INTERFACE_ON_MATCH_RECEIVED_TURN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TURN_BASED_MATCH_INTERFACE_ON_MATCH_ENDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTurnBasedMatchInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMatchReceivedTurn"),
            &raw mut U_TURN_BASED_MATCH_INTERFACE_ON_MATCH_RECEIVED_TURN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMatchEnded"),
            &raw mut U_TURN_BASED_MATCH_INTERFACE_ON_MATCH_ENDED,
        );
    }
}
#[repr(C, align(8))]
pub struct UNamedInterfaces {
    __padding_end: [u8; 112],
}
impl UNamedInterfaces {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamedInterfaces")
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
pub struct ITurnBasedMatchInterface {}
#[repr(C, align(8))]
pub struct UTurnBasedMatchInterface {
    __padding_end: [u8; 48],
}
impl UTurnBasedMatchInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTurnBasedMatchInterface")
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
pub struct EMPMatchOutcome(pub u8);
impl EMPMatchOutcome {
    pub const NONE: EMPMatchOutcome = EMPMatchOutcome(0);
    pub const QUIT: EMPMatchOutcome = EMPMatchOutcome(1);
    pub const WON: EMPMatchOutcome = EMPMatchOutcome(2);
    pub const LOST: EMPMatchOutcome = EMPMatchOutcome(3);
    pub const TIED: EMPMatchOutcome = EMPMatchOutcome(4);
    pub const TIME_EXPIRED: EMPMatchOutcome = EMPMatchOutcome(5);
    pub const FIRST: EMPMatchOutcome = EMPMatchOutcome(6);
    pub const SECOND: EMPMatchOutcome = EMPMatchOutcome(7);
    pub const THIRD: EMPMatchOutcome = EMPMatchOutcome(8);
    pub const FOURTH: EMPMatchOutcome = EMPMatchOutcome(9);
}
