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
    pub u_turn_based_match_interface_on_match_received_turn: *mut crate::ffi::UFunctionOpague,
    pub u_turn_based_match_interface_on_match_ended: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_turn_based_match_interface_on_match_received_turn: std::ptr::null_mut(),
            u_turn_based_match_interface_on_match_ended: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTurnBasedMatchInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMatchReceivedTurn"),
            &raw mut __FUNCTION_PTRS.u_turn_based_match_interface_on_match_received_turn,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMatchEnded"),
            &raw mut __FUNCTION_PTRS.u_turn_based_match_interface_on_match_ended,
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
    pub fn on_match_received_turn(
        &mut self,
        match_: FString,
        b_did_become_active: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem::__FUNCTION_PTRS
                    .u_turn_based_match_interface_on_match_received_turn,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&match_, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_did_become_active,
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
                crate::bindings::online_subsystem::__FUNCTION_PTRS
                    .u_turn_based_match_interface_on_match_received_turn,
                __buffer,
            )
        };
    }
    pub fn on_match_ended(&mut self, match_: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem::__FUNCTION_PTRS
                    .u_turn_based_match_interface_on_match_ended,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&match_, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem::__FUNCTION_PTRS
                    .u_turn_based_match_interface_on_match_ended,
                __buffer,
            )
        };
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
