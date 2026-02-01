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
#[repr(C, align(8))]
pub struct ULocalizationSettings {
    __padding_end: [u8; 96],
}
impl ULocalizationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizationSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizationSettings")
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
#[repr(C, align(8))]
pub struct ULocalizationTargetSet {
    __padding_end: [u8; 64],
}
impl ULocalizationTargetSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizationTargetSet")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizationTargetSet")
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
#[repr(C, align(8))]
pub struct ULocalizationTarget {
    __padding_end: [u8; 512],
}
impl ULocalizationTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizationTarget")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizationTarget")
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
#[repr(C, align(8))]
pub struct UUserGeneratedContentLocalizationSettings {
    __padding_end: [u8; 72],
}
impl UUserGeneratedContentLocalizationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserGeneratedContentLocalizationSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserGeneratedContentLocalizationSettings")
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
pub struct ELocalizationGatherPathRoot(pub u8);
impl ELocalizationGatherPathRoot {
    pub const AUTO: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(0);
    pub const ENGINE: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(1);
    pub const PROJECT: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(2);
}
#[repr(transparent)]
pub struct ELocalizedTextCollapseMode(pub u8);
impl ELocalizedTextCollapseMode {
    pub const IDENTICAL_TEXT_ID_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        0,
    );
    pub const IDENTICAL_PACKAGE_ID_TEXT_ID_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        1,
    );
    pub const IDENTICAL_NAMESPACE_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPortableObjectFormat(pub u8);
impl EPortableObjectFormat {
    pub const UNREAL: EPortableObjectFormat = EPortableObjectFormat(0);
    pub const CROWDIN: EPortableObjectFormat = EPortableObjectFormat(1);
}
#[repr(transparent)]
pub struct ELocalizationTargetConflictStatus(pub u8);
impl ELocalizationTargetConflictStatus {
    pub const UNKNOWN: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        0,
    );
    pub const CONFLICTS_PRESENT: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        1,
    );
    pub const CLEAR: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        2,
    );
}
