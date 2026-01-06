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
pub struct UAutomationControllerSettings {
    __padding_end: [u8; 152],
}
impl UAutomationControllerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomationControllerSettings")
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
pub struct EAutomationArtifactType(pub u8);
impl EAutomationArtifactType {
    pub const NONE: EAutomationArtifactType = EAutomationArtifactType(0);
    pub const IMAGE: EAutomationArtifactType = EAutomationArtifactType(1);
    pub const COMPARISON: EAutomationArtifactType = EAutomationArtifactType(2);
}
