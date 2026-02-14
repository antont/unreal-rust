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
pub struct UGraphEditorSettings {
    __padding_end: [u8; 808],
}
impl UGraphEditorSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGraphEditorSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGraphEditorSettings")
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
pub struct EGraphPanningMouseButton(pub u8);
impl EGraphPanningMouseButton {
    pub const RIGHT: EGraphPanningMouseButton = EGraphPanningMouseButton(0);
    pub const MIDDLE: EGraphPanningMouseButton = EGraphPanningMouseButton(1);
    pub const BOTH: EGraphPanningMouseButton = EGraphPanningMouseButton(2);
}
#[repr(transparent)]
pub struct EGraphZoomLimitHandling(pub u8);
impl EGraphZoomLimitHandling {
    pub const DEFAULT: EGraphZoomLimitHandling = EGraphZoomLimitHandling(0);
    pub const ALLOW_LIMIT_BREAK: EGraphZoomLimitHandling = EGraphZoomLimitHandling(1);
    pub const DISALLOW_LIMIT_BREAK: EGraphZoomLimitHandling = EGraphZoomLimitHandling(2);
}
