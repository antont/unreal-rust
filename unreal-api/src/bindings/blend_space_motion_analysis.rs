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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(16))]
pub struct UCachedMotionAnalysisProperties {
    __padding_end: [u8; 592],
}
impl UCachedMotionAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCachedMotionAnalysisProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCachedMotionAnalysisProperties")
            .copied()
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
pub struct ULocomotionAnalysisProperties {
    __padding_end: [u8; 416],
}
impl ULocomotionAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocomotionAnalysisProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocomotionAnalysisProperties")
            .copied()
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
pub struct URootMotionAnalysisProperties {
    __padding_end: [u8; 416],
}
impl URootMotionAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionAnalysisProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionAnalysisProperties")
            .copied()
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
pub struct EAnalysisLocomotionAxis(pub u8);
impl EAnalysisLocomotionAxis {
    pub const SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(0);
    pub const DIRECTION: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(1);
    pub const FORWARD_SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(2);
    pub const RIGHTWARD_SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(3);
    pub const UPWARD_SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(4);
    pub const FORWARD_SLOPE: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(5);
    pub const RIGHTWARD_SLOPE: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(6);
}
#[repr(transparent)]
pub struct EAnalysisRootMotionAxis(pub u8);
impl EAnalysisRootMotionAxis {
    pub const SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(0);
    pub const DIRECTION: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(1);
    pub const FORWARD_SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(2);
    pub const RIGHTWARD_SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(3);
    pub const UPWARD_SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(4);
    pub const FORWARD_SLOPE: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(5);
    pub const RIGHTWARD_SLOPE: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(6);
}
