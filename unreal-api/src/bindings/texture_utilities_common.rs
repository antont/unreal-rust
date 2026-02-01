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
pub struct UTextureImportSettings {
    __padding_end: [u8; 120],
}
impl UTextureImportSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureImportSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureImportSettings")
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
pub struct UTextureImportUserSettings {
    __padding_end: [u8; 112],
}
impl UTextureImportUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureImportUserSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureImportUserSettings")
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
pub struct ETextureImportFloatingPointFormat(pub u8);
impl ETextureImportFloatingPointFormat {
    pub const HDR_F16: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        0,
    );
    pub const HDR_COMPRESSED_BC6: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        1,
    );
    pub const HDR_F32_OR_F16: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        2,
    );
    pub const PREVIOUS_DEFAULT: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        0,
    );
}
#[repr(transparent)]
pub struct ETextureImportPNGInfill(pub u8);
impl ETextureImportPNGInfill {
    pub const DEFAULT: ETextureImportPNGInfill = ETextureImportPNGInfill(0);
    pub const NEVER: ETextureImportPNGInfill = ETextureImportPNGInfill(1);
    pub const ONLY_ON_BINARY_TRANSPARENCY: ETextureImportPNGInfill = ETextureImportPNGInfill(
        2,
    );
    pub const ALWAYS: ETextureImportPNGInfill = ETextureImportPNGInfill(3);
}
