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
pub struct UPIEPreviewDeviceSpecification {
    __padding_end: [u8; 208],
}
impl UPIEPreviewDeviceSpecification {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPIEPreviewDeviceSpecification")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPIEPreviewDeviceSpecification")
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
pub struct EPIEPreviewDeviceType(pub u8);
impl EPIEPreviewDeviceType {
    pub const UNSET: EPIEPreviewDeviceType = EPIEPreviewDeviceType(0);
    pub const ANDROID: EPIEPreviewDeviceType = EPIEPreviewDeviceType(1);
    pub const IOS: EPIEPreviewDeviceType = EPIEPreviewDeviceType(2);
    pub const TVOS: EPIEPreviewDeviceType = EPIEPreviewDeviceType(3);
    pub const SWITCH: EPIEPreviewDeviceType = EPIEPreviewDeviceType(4);
    pub const MAX: EPIEPreviewDeviceType = EPIEPreviewDeviceType(5);
}
