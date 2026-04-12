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
pub struct UWindowsTargetSettings {
    __padding_end: [u8; 272],
}
impl UWindowsTargetSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWindowsTargetSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWindowsTargetSettings")
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
pub struct EDefaultGraphicsRHI(pub u8);
impl EDefaultGraphicsRHI {
    pub const DEFAULT_GRAPHICS_RHI_DEFAULT: EDefaultGraphicsRHI = EDefaultGraphicsRHI(0);
    pub const DEFAULT_GRAPHICS_RHI_DX11: EDefaultGraphicsRHI = EDefaultGraphicsRHI(1);
    pub const DEFAULT_GRAPHICS_RHI_DX12: EDefaultGraphicsRHI = EDefaultGraphicsRHI(2);
    pub const DEFAULT_GRAPHICS_RHI_VULKAN: EDefaultGraphicsRHI = EDefaultGraphicsRHI(3);
}
#[repr(transparent)]
pub struct ECompilerVersion(pub u8);
impl ECompilerVersion {
    pub const DEFAULT: ECompilerVersion = ECompilerVersion(0);
    pub const VISUAL_STUDIO2015: ECompilerVersion = ECompilerVersion(1);
    pub const VISUAL_STUDIO2017: ECompilerVersion = ECompilerVersion(2);
    pub const VISUAL_STUDIO2019: ECompilerVersion = ECompilerVersion(3);
    pub const VISUAL_STUDIO2022: ECompilerVersion = ECompilerVersion(4);
    pub const VISUAL_STUDIO2026: ECompilerVersion = ECompilerVersion(5);
}
