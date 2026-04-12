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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UMetaHumanComponentBase {
    __padding_end: [u8; 448],
}
impl UMetaHumanComponentBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanComponentBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanComponentBase")
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
pub struct UMetaHumanComponentUE {
    __padding_end: [u8; 496],
}
impl UMetaHumanComponentUE {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanComponentUE")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanComponentUE")
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
pub struct EMetaHumanQualityLevel(pub u8);
impl EMetaHumanQualityLevel {
    pub const LOW: EMetaHumanQualityLevel = EMetaHumanQualityLevel(0);
    pub const MEDIUM: EMetaHumanQualityLevel = EMetaHumanQualityLevel(1);
    pub const HIGH: EMetaHumanQualityLevel = EMetaHumanQualityLevel(2);
    pub const CINEMATIC: EMetaHumanQualityLevel = EMetaHumanQualityLevel(3);
    pub const COUNT: EMetaHumanQualityLevel = EMetaHumanQualityLevel(4);
}
#[repr(transparent)]
pub struct EMetaHumanBodyType(pub u8);
impl EMetaHumanBodyType {
    pub const F_MED_NRW: EMetaHumanBodyType = EMetaHumanBodyType(0);
    pub const F_MED_OVW: EMetaHumanBodyType = EMetaHumanBodyType(1);
    pub const F_MED_UNW: EMetaHumanBodyType = EMetaHumanBodyType(2);
    pub const F_SRT_NRW: EMetaHumanBodyType = EMetaHumanBodyType(3);
    pub const F_SRT_OVW: EMetaHumanBodyType = EMetaHumanBodyType(4);
    pub const F_SRT_UNW: EMetaHumanBodyType = EMetaHumanBodyType(5);
    pub const F_TAL_NRW: EMetaHumanBodyType = EMetaHumanBodyType(6);
    pub const F_TAL_OVW: EMetaHumanBodyType = EMetaHumanBodyType(7);
    pub const F_TAL_UNW: EMetaHumanBodyType = EMetaHumanBodyType(8);
    pub const M_MED_NRW: EMetaHumanBodyType = EMetaHumanBodyType(9);
    pub const M_MED_OVW: EMetaHumanBodyType = EMetaHumanBodyType(10);
    pub const M_MED_UNW: EMetaHumanBodyType = EMetaHumanBodyType(11);
    pub const M_SRT_NRW: EMetaHumanBodyType = EMetaHumanBodyType(12);
    pub const M_SRT_OVW: EMetaHumanBodyType = EMetaHumanBodyType(13);
    pub const M_SRT_UNW: EMetaHumanBodyType = EMetaHumanBodyType(14);
    pub const M_TAL_NRW: EMetaHumanBodyType = EMetaHumanBodyType(15);
    pub const M_TAL_OVW: EMetaHumanBodyType = EMetaHumanBodyType(16);
    pub const M_TAL_UNW: EMetaHumanBodyType = EMetaHumanBodyType(17);
    pub const BLENDABLE_BODY: EMetaHumanBodyType = EMetaHumanBodyType(18);
    pub const COUNT: EMetaHumanBodyType = EMetaHumanBodyType(19);
}
