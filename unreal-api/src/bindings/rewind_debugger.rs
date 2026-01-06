#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(8))]
pub struct URewindDebuggerSettings {
    __padding_end: [u8; 152],
}
impl URewindDebuggerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URewindDebuggerSettings")
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
pub struct ERewindDebuggerCameraMode(pub u8);
impl ERewindDebuggerCameraMode {
    pub const REPLAY: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(0);
    pub const FOLLOW_TARGET_ACTOR: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(
        1,
    );
    pub const DISABLED: ERewindDebuggerCameraMode = ERewindDebuggerCameraMode(2);
}
